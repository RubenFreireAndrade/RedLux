use std::env;
use dotenv::dotenv;
use mongodb::{Client, bson::doc};


pub struct Database {
  //client_option: ClientOptions,
  //client: Client,
  database_uri: String,
  database_name: String,
  collection_name: String,
  //is_listen_submit_invoked: bool,
}

impl Database {
  pub fn database_default() -> Database {
    dotenv().ok();
    let mongo_username = env::var("MONGO_USERNAME").expect("MONGO_USERNAME was not found in .env");
    let mongo_password = env::var("MONGO_PASSWORD").expect("MONGO_PASSWORD was not found in .env");

    Database {
      database_uri: (format!("mongodb+srv://{mongo_username}:{mongo_password}@bookingform.woqmnkh.mongodb.net/")),
      database_name: ("redlux").to_string(),
      collection_name: ("bookings").to_string(),
    }
  }

  pub async fn connect_to_database(client: &Database) -> Client {
    let connected_client = match Client::with_uri_str(client.get_database_uri()).await {
      Ok(connected_client) => {
        println!("Connected to MongoDB");
        //Ok(connected_client)
        connected_client
      }
      Err(e) => {
        //println!("Error connecting to MongoDB: {}", e);
        //Err(e.into())
        panic!("{}", e)
      }
    };
    connected_client
  }

  // let connected_client = Client::with_uri_str(client.get_database_uri()).await.expect("Failed to connect to MongoDB");
  // connected_client
          
  pub async fn create_document(client: &Client, database: &Database, full_name: &str, location_of_collection: &str, location_of_destination: &str, phone_number: &str, important_information: &str) -> Result<String, String> {
    let database_name = client.database(database.get_database_name().as_str());
    let collection = database_name.collection(&database.get_collection_name());

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

  pub fn get_database_uri(&self) -> String {
      self.database_uri.clone()
  }

  pub fn get_database_name(&self) -> String {
      self.database_name.clone()
  }

  pub fn get_collection_name(&self) -> String {
      self.collection_name.clone()
  }
}