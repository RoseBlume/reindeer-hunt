mod filemanager;
mod manager;
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            filemanager::open,
            filemanager::save,
            filemanager::import,
            filemanager::save_times,
            filemanager::open_times,
            filemanager::generate_permits,
            filemanager::save_cache,
            filemanager::open_cache,
            manager::remove_student,
            manager::loss,
            manager::win,
            manager::pair_students,
            manager::next_round,
            manager::coin_toss,
            manager::sort_students,
            manager::single_toss,
            manager::reset_status,
            manager::update_notes,
            manager::add_student,
            manager::remove_lost,
            manager::end_program,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
