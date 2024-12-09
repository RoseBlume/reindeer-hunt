use tauri::command;
use rand::prelude::SliceRandom;
use rand::Rng;

// This command is used to remove a student from the list of students.
#[command]
pub fn remove_student(
    contents: serde_json::Value,
    name: &str,
    last_name: &str,
    home_room: &str,
) -> serde_json::Value {
    let mut students = contents.as_array().unwrap().clone();
    students.retain(|student| {
        student["firstName"] != name
            || student["lastName"] != last_name
            || student["room"] != home_room
    });
    serde_json::Value::Array(students)
}
// This command removes all students that have lost from the list of students.
#[command]
pub fn remove_lost(contents: serde_json::Value) ->  serde_json::Value {
    let mut students = contents.as_array().unwrap().clone();
    students.retain(|student| student["win"] == "win");
    for student in &mut students {
        student["pairFirst"] = serde_json::Value::String("".to_string());
        student["pairLast"] = serde_json::Value::String("".to_string());
        student["pairRoom"] = serde_json::Value::String("".to_string());
        student["win"] = serde_json::Value::String("undecided".to_string());
        student["pending"] = serde_json::Value::Bool(true);
    }
    serde_json::Value::Array(students)
}
// This command removes all students that have won from the list of students.
#[command]
pub fn loss(
    content: serde_json::Value,
    name: &str,
    last_name: &str,
    home_room: &str,
) -> serde_json::Value {
    let mut students = content.as_array().unwrap().clone();

    for student in &mut students {
        if student["pairFirst"] == name
            && student["pairLast"] == last_name
            && student["pairRoom"] == home_room
        {
            student["pending"] = serde_json::Value::Bool(false);
            student["win"] = serde_json::Value::String("win".to_string());
        }
        if student["firstName"] == name
            && student["lastName"] == last_name
            && student["room"] == home_room
        {
            student["pending"] = serde_json::Value::Bool(false);
            student["win"] = serde_json::Value::String("loss".to_string());
        }
    }

    serde_json::Value::Array(students)
}
// This command removes all students that have lost from the list of students.
#[command]
pub fn win(
    content: serde_json::Value,
    name: &str,
    last_name: &str,
    home_room: &str,
) -> serde_json::Value {
    let mut students = content.as_array().unwrap().clone();

    for student in &mut students {
        if student["pairFirst"] == name
            && student["pairLast"] == last_name
            && student["pairRoom"] == home_room
        {
            student["pending"] = serde_json::Value::Bool(false);
            student["win"] = serde_json::Value::String("loss".to_string());
        }
        if student["firstName"] == name
            && student["lastName"] == last_name
            && student["room"] == home_room
        {
            student["pending"] = serde_json::Value::Bool(false);
            student["win"] = serde_json::Value::String("win".to_string());
        }
    }

    serde_json::Value::Array(students)
}
// This command resets the status of a student to pending and undecided.
#[command]
pub fn reset_status(
    contents: serde_json::Value,
    name: &str,
    last_name: &str,
    home_room: &str,
) -> serde_json::Value {
    let mut students = contents.as_array().unwrap().clone();

    for student in &mut students {
        if student["firstName"] == name
            && student["lastName"] == last_name
            && student["room"] == home_room
        {
            student["pending"] = serde_json::Value::Bool(true);
            student["win"] = serde_json::Value::String("undecided".to_string());
        }
    }

    serde_json::Value::Array(students)
}


// This command pairs students together.
#[command]
pub fn pair_students(contents: serde_json::Value) -> serde_json::Value {
    let mut students = contents.as_array().unwrap().clone();
    let mut rng = rand::thread_rng();

    students.shuffle(&mut rng);

    let mut paired_students = Vec::new();

    while students.len() >= 2 {
        let mut student1 = students.pop().unwrap();
        let mut student2 = students.pop().unwrap();

        student1["pairFirst"] = student2["firstName"].clone();
        student1["pairLast"] = student2["lastName"].clone();
        student1["pairRoom"] = student2["room"].clone();

        student2["pairFirst"] = student1["firstName"].clone();
        student2["pairLast"] = student1["lastName"].clone();
        student2["pairRoom"] = student1["room"].clone();

        paired_students.push(student1);
        paired_students.push(student2);
    }

    // If there's an odd number of students, push the last one back to the list
    if !students.is_empty() {
        paired_students.push(students.pop().unwrap());
    }

    serde_json::Value::Array(paired_students)
}
// This command performs a coin toss on all students.
#[command]
pub fn coin_toss(contents: serde_json::Value) -> serde_json::Value {
    let mut students = contents.as_array().unwrap().clone();
    let mut rng = rand::thread_rng();

    for student in &students.clone() {
        if student["pending"].as_bool().unwrap() {
            let first_name = student["firstName"].as_str().unwrap();
            let last_name = student["lastName"].as_str().unwrap();
            let room = student["room"].as_str().unwrap();
            

            let toss: bool = rng.gen();
            if toss {
                students = win(serde_json::Value::Array(students.clone()), first_name, last_name, room).as_array().unwrap().clone();
            } else {
                students = loss(serde_json::Value::Array(students.clone()), first_name, last_name, room).as_array().unwrap().clone();
            }
        }
    }

    serde_json::Value::Array(students)
}
// This command performs a single coin toss on a student.
#[command]
pub fn single_toss(
    contents: serde_json::Value,
    name1: &str,
    last_name1: &str,
    home_room1: &str,
) -> serde_json::Value {
    let mut students = contents.as_array().unwrap().clone();
    let mut rng = rand::thread_rng();

    let toss: bool = rng.gen();
    if toss {
        students = win(serde_json::Value::Array(students.clone()), name1, last_name1, home_room1).as_array().unwrap().clone();
    } else {
        students = loss(serde_json::Value::Array(students.clone()), name1, last_name1, home_room1).as_array().unwrap().clone();
    }

    serde_json::Value::Array(students)
}
// This command starts the next round
#[command]
pub fn next_round(contents: serde_json::Value) -> serde_json::Value {
    let mut students = contents.as_array().unwrap().clone();

    // Perform coin toss on all students
    students = coin_toss(serde_json::Value::Array(students.clone())).as_array().unwrap().clone();

    // Reset status for all students
    for student in &students.clone() {
        let first_name = student["firstName"].as_str().unwrap();
        let last_name = student["lastName"].as_str().unwrap();
        let room = student["room"].as_str().unwrap();
        students = reset_status(serde_json::Value::Array(students.clone()), first_name, last_name, room).as_array().unwrap().clone();
    }

    // Remove students whose win variable is equal to "loss"
    students.retain(|student| student["win"] != "loss");

    // Pair remaining students
    students = pair_students(serde_json::Value::Array(students)).as_array().unwrap().clone();

    // Sort students
    students = sort_students(serde_json::Value::Array(students)).as_array().unwrap().clone();

    serde_json::Value::Array(students)
}
// This command sorts the students by homeroom, last name, and then first name.
#[command]
pub fn sort_students(contents: serde_json::Value) -> serde_json::Value {
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
// This command adds a student to the list of students.
#[command]
pub fn add_student(
    contents: serde_json::Value,
    first_name: &str,
    last_name: &str,
    home_room: &str,
    notes: &str,
) -> serde_json::Value {
    let mut students = contents.as_array().unwrap().clone();

    let new_student = serde_json::json!({
        "firstName": first_name,
        "lastName": last_name,
        "room": home_room,
        "notes": notes,
        "pending": true,
        "win": "undecided",
        "pairFirst": "",
        "pairLast": "",
        "pairRoom": ""
    });

    students.push(new_student);

    // Sort students before returning
    sort_students(serde_json::Value::Array(students))
}
// This command updates the notes of a student.
#[command]
pub fn update_notes(
    contents: serde_json::Value,
    name: &str,
    last_name: &str,
    home_room: &str,
    new_notes: &str,
) -> serde_json::Value {
    let mut students = contents.as_array().unwrap().clone();

    for student in &mut students {
        if student["firstName"] == name
            && student["lastName"] == last_name
            && student["room"] == home_room
        {
            student["notes"] = serde_json::Value::String(new_notes.to_string());
        }
    }

    serde_json::Value::Array(students)
}
// This command ends the program.
#[command]
pub fn end_program() {
    std::process::exit(0);
}