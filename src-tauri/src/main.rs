// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use dotenv::dotenv;
use mongodb::bson::doc;
use mongodb::Client;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
// #[tauri::command]
// fn trying_to_understand(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }

struct Database {
    //client_option: ClientOptions,
    client: Client,
    database_name: String,
    collection_name: String,
}

impl Database {
    fn get_client(&self) -> Client {
        self.client.clone()
    }

    // fn get_client_option(&self) -> ClientOptions {
    //     self.client_option.clone()
    // }

    fn get_database_name(&self) -> String {
        self.database_name.clone()
    }

    fn get_collection_name(&self) -> String {
        self.collection_name.clone()
    }
}


#[tauri::command]
fn trying_to_understand() {
    println!("I was invoked from JS!");
}

#[tauri::command]
fn test() {
    println!("Testing multiple commands");
}

#[tauri::command]
async fn create_document(client: &Client, db_name: &str, collection_name: &str, full_name: &str, location_of_collection: &str, location_of_destination: &str, phone_number: &str, important_information: &str) -> Result<String, String> {
    let db = client.database(db_name);
    let collection = db.collection(collection_name);

    let customer = doc! {
        "full_name": full_name,
        "location_of_collection": location_of_collection,
        "location_of_destination": location_of_destination,
        // "date_time_of_collection": "22/08/2023 | 22:00",
        // "date_time_of_destination": "02/10/2023 | 03:00",
        "phone_number": phone_number,
        "important_information": important_information,
    };

    collection.insert_one(customer, None).await.unwrap();

    Ok(("create_customer function was used").to_string())
}

#[tauri::command]
fn listen_submit(form_data: serde_json::Value) {
    println!("incoming data: {}", form_data);
}

// fn insert_document_test() {
//     let document = doc! {
//         "full_name": "Alice Smith",
//         "location_of_collection": "London",
//         "location_of_destination": "Gatwick",
//         "date_time_of_collection": "22/08/2023 | 22:00",
//         "date_time_of_destination": "02/10/2023 | 03:00",
//         "phone_number": "909019283192",
//         "important_information": "Call customer when close",
//     };
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    dotenv().ok();
    let mongo_username = env::var("MONGO_USERNAME").expect("MONGO_USERNAME was not found in .env");
    let mongo_password = env::var("MONGO_PASSWORD").expect("MONGO_PASSWORD was not found in .env");

    let database = Database {
        client: Client::with_uri_str(format!("mongodb+srv://{mongo_username}:{mongo_password}@bookingform.woqmnkh.mongodb.net/")).await?,
        database_name: "redlux".to_string(),
        collection_name: "bookings".to_string(),
    };

    // THIS IS USED TO CONNECT TO MONGODB
    // let client_options = ClientOptions::parse(format!("mongodb+srv://{mongo_username}:{mongo_password}@bookingform.woqmnkh.mongodb.net/")).await?;
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
    
    //create_document(&client, "redlux", "bookings", "Rubs").await?;
    


    // let desired_db = client.database("redlux");
    //let desired_collection = db.collection("bookings");

    //let desired_collection = desired_db.collection::<Document>("bookings");
    

    //desired_collection.insert_one(insert_document, None).await?;



    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![test, trying_to_understand, listen_submit])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
    
    Ok(())
}
