extern crate bson;
extern crate mongodb;

use mongodb::{Client, ThreadedClient};

pub mod repository;

fn get_mongo_client() -> mongodb::Client {
    let client = Client::connect("192.168.0.2", 27017)
        .ok().expect("Failed to initialize standalone client.");

    client
}
