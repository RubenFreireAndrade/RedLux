// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;

use std::env;
use dotenv::dotenv;
use mongodb::Client;
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

// #[tauri::command]
// fn listen_submit(form_data: serde_json::Value) {
//     println!("incoming data: {}", form_data);
    
//     // if !form_data.to_string().is_empty() {
//         //     //create_document(database, db_name, collection_name, full_name, location_of_collection, location_of_destination, phone_number, important_information)
//         // };
// }
    
// async fn create_document(database: &Client, db_name: &str, collection_name: &str, full_name: &str, location_of_collection: &str, location_of_destination: &str, phone_number: &str, important_information: &str) -> Result<String, String> {
//     let database_name = database.database(db_name);
//     let collection = database_name.collection(collection_name);

//     let customer = doc! {
//         "full_name": full_name,
//         "location_of_collection": location_of_collection,
//         "location_of_destination": location_of_destination,
//         // "date_time_of_collection": "22/08/2023 | 22:00",
//         // "date_time_of_destination": "02/10/2023 | 03:00",
//         "phone_number": phone_number,
//         "important_information": important_information,
//     };

//     collection.insert_one(customer, None).await.unwrap();

//     Ok(("create_customer function was used").to_string())
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database = db::Database::database_default();

    // let connected_database = match Client::with_uri_str(database.get_database_uri()).await {
    //     Ok(connected_database) => {
    //         println!("Connected to MongoDB");
    //         connected_database;
    //     }
    //     Err(e) => {
    //         println!("Error connecting to MongoDB: {}", e);
    //         return Ok(());
    //     }
    // };

    let connected_database = db::Database::connect_to_database(&database).await;

    //let create_customer = db::Database::create_document(&connected_database, &database, "Rubs", "location_of_collection", "location_of_destination", "phone_number", "important_information").await;

    // THIS IS USED TO CONNECT TO MONGODB
    // let client_options = ClientOptions::parse(format!("mongodb+srv://{mongo_username}:{mongo_password}@bookingform.woqmnkh.mongodb.net/")).await?;
    // let client = match Client::with_options(client_options) {
    //     Ok(connected_database) => {
    //         println!("Connected to MongoDB");
    //         connected_database
    //     }
    //     Err(e) => {
    //         println!("Error connecting to MongoDB: {}", e);
    //         return Ok(());
    //     }
    // };


    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![test, trying_to_understand /*listen_submit*/])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
    
    Ok(())
}
