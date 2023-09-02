// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;

use std::env;
use mongodb::bson::doc;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
// #[tauri::command]
// fn trying_to_understand(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }

#[tauri::command]
fn trying_to_understand() {
    println!("I was invoked from JS!");
}

#[tauri::command]
fn test() {
    println!("Testing multiple commands");
}

#[tauri::command]
// async fn listen_submit(client: &Client, database: &Database, form_data: serde_json::Value) {
//     println!("incoming data: {}", form_data);

//     let full_name = form_data["full_name"].to_string();
//     let location_of_collection = form_data["location_of_collection"].to_string();
//     let location_of_destination = form_data["location_of_destination"].to_string();
//     let phone_number = form_data["phone_number"].to_string();
//     let important_information = form_data["important_information"].to_string();

//     db::Database::create_document(client, database, &full_name, &location_of_collection, &location_of_destination, &phone_number, &important_information).await;
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database = db::Database::database_default();

    let connected_database = db::Database::connect_to_database(&database).await;

    

    //let create_customer = db::Database::create_document(&connected_database, &database, "Rubs", "location_of_collection", "location_of_destination", "phone_number", "important_information").await;

    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![test, trying_to_understand, /*listen_submit*/])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
    
    Ok(())
}
