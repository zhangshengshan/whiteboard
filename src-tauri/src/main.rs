use tauri::Manager;
use std::fs::File;
use std::io::Write;


// tauri::Builder::default()
  // .setup(|app| {
    // #[cfg(debug_assertions)] // only include this code on debug builds
    // {
      // let window = app.get_window("main").unwrap();
      // window.open_devtools();
      // window.close_devtools();
    // }
    // Ok(())
  // });


#[tauri::command]
fn greet(name: &str, filepath: &str) -> String {
    println!("----------------------------------");
    println!("filePath {}", filepath);
    println!("{}", name);
    let mut file = std::fs::File::create(filepath).expect("create failed");
    file.write_all(name.as_bytes()).expect("write failed");
    println!("----------------------------------");
    format!("{}", name)

}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
