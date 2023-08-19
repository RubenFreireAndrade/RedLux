// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use mongodb::bson::{doc, Document};
use mongodb::{Client, options::ClientOptions};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
// #[tauri::command]
// fn trying_to_understand(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }

#[tauri::command]
fn trying_to_understand() {
    println!("I was invoked from JS!");
}

// async fn create_collection(client: &Client, db_name: &str, collection_name: &str) {
//     let db = client.database(db_name);
//     db.create_collection(collection_name, None).await;
// }

// async fn insert_document(client: &Client, db_name: &str, collection_name: &str) {
//     let db = client.database(db_name);
//     let collection = db.collection(collection_name);

//     let doc = doc! { "name": "John", "age": 30 };

//     collection.insert_one(doc, None).await.unwrap();
// }

fn insert_document() {
    let document = doc! {
        "full_name": "Alice Smith",
        "location_of_collection": "London",
        "location_of_destination": "Gatwick",
        "date_time_of_collection": "22/08/2023 | 22:00",
        "date_time_of_destination": "02/10/2023 | 03:00",
        "phone_number": "909019283192",
        "important_information": "Call customer when close",
    };
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let client_options = ClientOptions::parse("mongodb+srv://GAMUUz:Lordyama45@bookingform.woqmnkh.mongodb.net/").await?;

    // let client = match Client::with_options(client_options) {
    //     Ok(client) => {
    //         println!("Connected to MongoDB");
    //         client
    //     }
    //     Err(e) => {
    //         println!("Error connecting to MongoDB: {}", e);
    //         return Ok(());
    //     }
    // };
    
    // let desired_db = client.database("redlux");
    //let desired_collection = db.collection("bookings");

    //let desired_collection = desired_db.collection::<Document>("bookings");
    

    //desired_collection.insert_one(insert_document, None).await?;


    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![trying_to_understand])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
    
    Ok(())
}
