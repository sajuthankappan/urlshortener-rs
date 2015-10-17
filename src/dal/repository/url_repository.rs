extern crate mongodb;
extern crate bson;
extern crate urlshortener_codec as codec;

use dal;
use models;
use bson::Bson;
use mongodb::{ThreadedClient};
use mongodb::db::ThreadedDatabase;
use super::CounterRepository;

pub struct UrlRepository;

impl UrlRepository {
    pub fn new() -> UrlRepository {
        UrlRepository
    }

    pub fn find_one(&self, alias: String) -> Option<models::Url> {
        let coll = self.get_urls_collection();
        let id = alias.clone();
        let doc = doc! { "_id" => alias };

        let mut cursor = coll.find(Some(doc.clone()), None)
            .ok().expect("Failed to execute find.");

        let item = cursor.next();

        let long_url: String;

        match item {
            Some(Ok(doc)) => match doc.get("longUrl") {
                Some(&Bson::String(ref u)) => long_url = u.to_string(),
                _ => panic!("Expected longUrl to be a string!"),
            },
            //Some(Ok(doc)) => long_url = doc.get("LongUrl").unwrap().to_json().as_string().unwrap().to_string(),
            Some(Err(_)) => panic!("Failed to get next from server!"),
            None => panic!("Server returned no results!"),
        };

        Some(models::Url {
            id: Some(id),
            long_url: long_url
        })
    }

    pub fn add(&self, url: &mut models::Url) -> Result<bool, &str> {
        let coll = self.get_urls_collection();
        let long_url = url.long_url.clone();

        let doc: bson::Document;
        if let Some(id) = url.id.clone() {
            doc = doc! { "_id" => id,
                             "longUrl" => long_url };
        }
        else {
            let num = CounterRepository::new().increment_counter("Url");
            let id = codec::encode(num);
            //let url.id = Some(id);
            doc = doc! { "_id" => id,
                         "longUrl" => long_url };
        }

        let _ = coll.insert_one(doc, None).unwrap();

        Ok(true)
    }

    fn get_urls_collection(&self) -> mongodb::coll::Collection {
        let client = dal::get_mongo_client();
        let coll = client.db("urlshortener").collection("urls");
        coll
    }
}