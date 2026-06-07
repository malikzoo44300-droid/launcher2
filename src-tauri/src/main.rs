// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tokio::main]
async fn main() {
    retrac_launcher_lib::run(vec![]);
}
fn main() {
  // Ajoute cette ligne avant le builder
  std::env::set_var("WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS", "--disable-gpu --disable-software-rasterizer");

  tauri::Builder::default()
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
