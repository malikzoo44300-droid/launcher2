#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tokio::main]
async fn main() {
    // 1. Définition des variables d'environnement
    std::env::set_var("WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS", "--disable-gpu --disable-software-rasterizer");
    
    // 2. Récupération des arguments et appel de la bibliothèque
    let args: Vec<String> = std::env::args().collect();
    retrac_launcher_lib::run(args);
    
}