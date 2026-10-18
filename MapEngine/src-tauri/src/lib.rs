#![allow(unused_variables,unused)]


// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::{fs,path::PathBuf};
mod map;
mod thing;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, list_maps])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn list_maps() -> Result<Vec<String>, String>
{
        let maps_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("maps");

    let entries = std::fs::read_dir(&maps_dir)
        .map_err(|e| format!("Nao foi possivel encontrar mapas"))?;

    let mut maps : Vec<String> = Vec::new();

    for entry in entries
    {
            let entry = entry
        .map_err(|e| format!("Erro ao ler entrada: {e}"))?;

        let path = entry.path();
        if !path.is_file() || !path.extension().is_some_and(|ext| ext == "json") 
        {continue};
        
        if let Some(name) = path.file_stem() {
            maps.push(name.to_string_lossy().into_owned());
        }
     }

    maps.sort();
    Ok(maps)
}
