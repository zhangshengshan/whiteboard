use tauri::Manager;

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
fn greet(name: &str) -> String {
    println!("----------------------------------");
    println!("{}", name);
    println!("----------------------------------");
    format!("{}", name)
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
