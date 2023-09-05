use dotenv::dotenv;
use mongodb::{bson::doc, Client, Database};
use std::sync::Mutex;
use std::{env, sync::MutexGuard};

use crate::form_data;

pub static mut DATABASE: Option<Mutex<Database>> = None;

pub async fn initialize() {
    dotenv().ok();
    let mongo_username = env::var("MONGO_USERNAME").expect("MONGO_USERNAME was not found in .env");
    let mongo_password = env::var("MONGO_PASSWORD").expect("MONGO_PASSWORD was not found in .env");

    let client = Client::with_uri_str(&format!(
        "mongodb+srv://{mongo_username}:{mongo_password}@bookingform.woqmnkh.mongodb.net/"
    ))
    .await
    .expect("Failed to initialize client.");

    let database = client.database("redlux");

    unsafe {
        DATABASE = Some(Mutex::new(database));
    }
}

pub fn get_database() -> MutexGuard<'static, Database> {
    unsafe {
		DATABASE.as_ref().unwrap().lock().unwrap()
	}
}

pub async fn create_document(form_data: form_data::FormData) {
    let collection = get_database().collection("bookings");

    let customer = doc! {
      "full_name": form_data.full_name,
      "location_of_collection": form_data.location_of_collection,
      "location_of_destination": form_data.location_of_destination,
      // "date_time_of_collection": "22/08/2023 | 22:00",
      // "date_time_of_destination": "02/10/2023 | 03:00",
      "phone_number": form_data.phone_number,
      "important_information": form_data.important_information,
    };

    collection.insert_one(customer, None).await.unwrap();
}
