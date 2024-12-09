use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::fs;
use genpdf::{fonts, Element, Alignment};
use genpdf::elements::{Paragraph, PageBreak, Break};
use genpdf::style::Style;
use whoami;
use shellexpand;
#[cfg(target_os = "linux")]
const FONT_DIRS: &[&str] = &[
    "/usr/share/fonts/liberation",
    "/usr/share/fonts/truetype/liberation",
];

#[cfg(target_os = "windows")]
const FONT_DIRS: &[&str] = &[
    "C:\\Windows\\Fonts",
];
const DEFAULT_FONT_NAME: &'static str = "LiberationSans";

// Paths for windows
#[cfg(target_os = "windows")]
fn get_cache_path() -> String{
    format!("C:\\Users\\{}\\AppData\\Roaming\\reindeer-hunt\\cache.json", whoami::username().to_string())
}
#[cfg(target_os = "windows")]
fn get_config_path() -> String{
    format!("C:\\Users\\{}\\AppData\\Roaming\\reindeer-hunt\\config.json", whoami::username().to_string())
}

// Paths for Linux
#[cfg(target_os = "linux")]
fn get_cache_path() -> String{
    format!("/home/{}/.local/reindeer-hunt/cache.json", whoami::username().to_string())
}
#[cfg(target_os = "linux")]
fn get_config_path() -> String{
    format!("/home/{}/.local/reindeer-hunt/config.json", whoami::username().to_string())
}


#[tauri::command]
pub fn save_times(contents: serde_json::Value) {
    let config_path = get_config_path();
    let config_path = shellexpand::tilde(&config_path).to_string();
    save(&config_path, contents.clone());
}
// This command runs on startup and returns the json code from the config file. If the file does not exist, it creates it and sets the default times.
#[tauri::command]
pub fn open_times() -> serde_json::Value {
    let config_path = get_config_path();
    let config_path = shellexpand::tilde(&config_path).to_string();
    let config_dir = std::path::Path::new(&config_path).parent().unwrap();
    let default_times = serde_json::json!([
        {
            "start": "8:00AM",
            "end": "8:15AM"
        },
        {
            "start": "11:00AM",
            "end": "11:50AM"
        },
        {
            "start": "2:30PM",
            "end": "3:00PM"
        }
    ]);
    if !config_dir.exists() {
        fs::create_dir_all(config_dir).expect("Unable to create config directory");
    }

    if !std::path::Path::new(&config_path).exists() {
        save(&config_path, default_times.clone());
        default_times
    }
    else {
        open(&config_path)
    }
}

// This command imports a csv file and returns the json code for the students.
#[tauri::command]
pub fn import(path: &str) -> serde_json::Value {
    let file = File::open(path).expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut students = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        let fields: Vec<&str> = line.split(',').collect();
        if fields.len() == 4 {
            if fields[1] != "Usual name" {
                let student = serde_json::json!({
                    "firstName": fields[1],
                    "lastName": fields[0],
                    "room": fields[2],
                    "pending": true,
                    "win": "undecided",
                    "notes": "",
                    "rand": 0,
                    "pairFirst": "",
                    "pairLast": "",
                    "pairRoom": ""
                });
                students.push(student);
            }
        }
    }

    sort_students(serde_json::Value::Array(students))
}

// This command saves the json code to a file.
#[tauri::command]
pub fn save(path: &str, data_info: serde_json::Value) {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)
        .expect("Unable to open file");

    let data = serde_json::to_string(&data_info).expect("Unable to serialize JSON data");
    file.write_all(data.as_bytes())
        .expect("Unable to write data to file");
}
// This command opens a file and returns the json code.
#[tauri::command]
pub fn open(path: &str) -> serde_json::Value {
    let contents = fs::read_to_string(path).expect("Unable to read file");
    serde_json::from_str(&contents).expect("Unable to parse JSON")
}
// This command saves the json code to a file.
#[tauri::command]
pub fn save_cache(contents: serde_json::Value) {
    let cache_path = get_cache_path();
    let config_path = shellexpand::tilde(&cache_path).to_string();
    save(&config_path, contents.clone());
}

// This command runs on startup and returns the json code from the cache file. If the file does not exist, it creates it and sets the default students.
#[tauri::command]
pub fn open_cache() -> serde_json::Value {
    let cache_path = get_cache_path();
    let config_path = shellexpand::tilde(&cache_path).to_string();
    let config_dir = std::path::Path::new(&config_path).parent().unwrap();
    
    let default_students = serde_json::json!([
        {
            // Students name
            "firstName": "",
            "lastName": "",
            // The home room number or spare
            "room": "",
            "rand": 0,
            "win": "",
            "notes": "",
            "pending": true,
            // The name of the opposing student
            "pairFirst": "",
            "pairLast": "",
            "pairRoom": ""
        }
    ]);
    if !config_dir.exists() {
        fs::create_dir_all(config_dir).expect("Unable to create config directory");
    }

    if !std::path::Path::new(&config_path).exists() {
        save(&config_path, default_students.clone());
        default_students
    }
    else {
        open(&config_path)
    }
}

// This command sorts the students by room number, last name, and first name.
fn sort_students(contents: serde_json::Value) -> serde_json::Value {
    let mut students = contents.as_array().unwrap().clone();

    students.sort_by(|a, b| {
        let room_cmp = a["room"].as_str().unwrap().cmp(b["room"].as_str().unwrap());
        if room_cmp == std::cmp::Ordering::Equal {
            let last_name_cmp = a["lastName"].as_str().unwrap().cmp(b["lastName"].as_str().unwrap());
            if last_name_cmp == std::cmp::Ordering::Equal {
                a["firstName"].as_str().unwrap().cmp(b["firstName"].as_str().unwrap())
            } else {
                last_name_cmp
            }
        } else {
            room_cmp
        }
    });

    serde_json::Value::Array(students)
}

// This command generates the permits for the students and saves them to a pdf file.
#[tauri::command]
pub fn generate_permits(contents: serde_json::Value, path: &str, times: serde_json::Value) {
    let times_array = times.as_array().expect("Times should be an array");
    let morning_time = &times_array[0];
    let lunch_time = &times_array[1];
    let evening_time = &times_array[2];

    let morning_time_vec = vec![
        morning_time["start"].as_str().unwrap().to_string(),
        morning_time["end"].as_str().unwrap().to_string(),
    ];
    let lunch_time_vec = vec![
        lunch_time["start"].as_str().unwrap().to_string(),
        lunch_time["end"].as_str().unwrap().to_string(),
    ];
    let evening_time_vec = vec![
        evening_time["start"].as_str().unwrap().to_string(),
        evening_time["end"].as_str().unwrap().to_string(),
    ];
    let font_dir = FONT_DIRS
        .iter()
        .filter(|path| std::path::Path::new(path).exists())
        .next()
        .expect("Could not find font directory");
    let default_font =
        fonts::from_files(font_dir, DEFAULT_FONT_NAME, Some(fonts::Builtin::Helvetica))
            .expect("Failed to load the default font family");

    let mut doc = genpdf::Document::new(default_font);
    // Create a document and set the default font family
    // Change the default settings
    doc.set_title("Reindeer Hunt Permits");
    #[cfg(target_os = "linux")]
    doc.set_paper_size(genpdf::PaperSize::Letter);
    // Customize the pages
    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(10);
    doc.set_page_decorator(decorator);
    // Add one or more elements
    // Render the document and write it to a file

    let mut students = contents.as_array().unwrap().clone();

    students.sort_by(|a, b| {
        let room_cmp = a["room"].as_str().unwrap().cmp(b["room"].as_str().unwrap());
        if room_cmp == std::cmp::Ordering::Equal {
            let last_name_cmp = a["lastName"].as_str().unwrap().cmp(b["lastName"].as_str().unwrap());
            if last_name_cmp == std::cmp::Ordering::Equal {
                a["firstName"].as_str().unwrap().cmp(b["firstName"].as_str().unwrap())
            } else {
                last_name_cmp
            }
        } else {
            room_cmp
        }
    });
    let mut current_room = String::new();
    let mut student_count = 0;
    doc.set_minimal_conformance();
    for student in students {
        let room = student["room"].as_str().unwrap().to_string();
        if room != current_room {
            current_room = room.clone();
            let header = format!("Homeroom: {}", current_room);
            let texter = PageBreak::new();
            let texten = Break::new(1);
            doc.push(texter);
            doc.push(genpdf::elements::Paragraph::new(header).styled(genpdf::style::Style::new().bold()));
            doc.push(texten);
            student_count = 0;
        }
        if student_count > 0 && student_count % 11 == 0 {
            doc.push(PageBreak::new());
        }
        student_count += 1;

        let room = student["room"].as_str().unwrap().to_string();
        if room != current_room {
            current_room = room.clone();
            let header = format!("Homeroom: {}", current_room);
            doc.push(genpdf::elements::Paragraph::new(header));
        }
        let text = format!(
            "This permit gives you {}, {} of homeroom {},",
            student["firstName"].as_str().unwrap(),
            student["lastName"].as_str().unwrap(),
            student["room"].as_str().unwrap(),
        );
        let text2 = format!(
            "permission to hunt {}, {} of homeroom {},",
            student["pairFirst"].as_str().unwrap(),
            student["pairLast"].as_str().unwrap(),
            student["pairRoom"].as_str().unwrap(),
        );
        let text3 = format!(
            "between the hours of {}-{}, {}-{}, and ",
            morning_time_vec[0],
            morning_time_vec[1],
            lunch_time_vec[0],
            lunch_time_vec[1],
        );
        let text4 = format!(
            "{}-{}",
            evening_time_vec[0],
            evening_time_vec[1]
        );
//        doc.push(genpdf::elements::Paragraph::new(text).aligned(Alignment::Center));
        doc.push(
            Paragraph::new(text)
                .aligned(Alignment::Left)
                .styled(Style::new().with_font_size(11)),
        );
        doc.push(
            Paragraph::new(text2)
                .aligned(Alignment::Left)
                .styled(Style::new().with_font_size(11)),
        );
        doc.push(
            Paragraph::new(text3)
                .aligned(Alignment::Left)
                .styled(Style::new().with_font_size(11)),
        );
        doc.push(
            Paragraph::new(text4)
                .aligned(Alignment::Left)
                .styled(Style::new().with_font_size(11)),
        );
        doc.push(Break::new(1));
    }

    doc.render_to_file(path).expect("Failed to write PDF file");
}