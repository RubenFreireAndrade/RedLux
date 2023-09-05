// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod form_data;

use std::env;
use mongodb::bson::doc;

#[tauri::command]
async fn listen_submit(form_data: serde_json::Value) {
    println!("incoming data: {}", form_data);

    let customer_data = form_data::FormData {
        full_name: form_data["full_name"].to_string(),
        location_of_collection: form_data["location_of_collection"].to_string(),
        location_of_destination: form_data["location_of_destination"].to_string(),
        phone_number: form_data["phone_number"].to_string(),
        important_information: form_data["important_information"].to_string(),
    };

    db::create_document(customer_data).await;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    db::initialize().await;

    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![listen_submit])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
    
    Ok(())
}
