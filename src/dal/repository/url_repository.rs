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
            None => {return None},
        };

        Some(models::Url {
            id: Some(id),
            long_url: long_url
        })
    }

    pub fn find_by_long_url(&self, long_url: String) -> Option<models::Url> {
        let coll = self.get_urls_collection();
        let url = long_url.clone();
        let doc = doc! { "longUrl" => url };

        let mut cursor = coll.find(Some(doc.clone()), None)
            .ok().expect("Failed to execute find.");

        let item = cursor.next();

        let id: String;

        match item {
            Some(Ok(doc)) => match doc.get("_id") {
                Some(&Bson::String(ref i)) => id = i.to_string(),
                _ => panic!("Expected longUrl to be a string!"),
            },
            //Some(Ok(doc)) => long_url = doc.get("LongUrl").unwrap().to_json().as_string().unwrap().to_string(),
            Some(Err(_)) => panic!("Failed to get next from server!"),
            None => {return None},
        };

        Some(models::Url {
            id: Some(id),
            long_url: long_url
        })
    }

    pub fn add(&self, mut url: models::Url) -> Result<models::Url, &str> {
        let coll = self.get_urls_collection();
        let long_url = url.long_url.clone();

        let find = self.find_by_long_url(long_url.clone());

        if let Some(url) = find {
            return Ok(url);
        }

        if let None = url.id {
            let num = CounterRepository::new().increment_counter("Url");
            url.id = Some(codec::encode(num));
        }
        let id = url.id.clone().unwrap();
        let doc = doc! { "_id" => id,
                         "longUrl" => long_url };

        let insert_result = coll.insert_one(doc, None);
        let result = match insert_result {
            Ok(v) => match v.inserted_id {
                None => Err("Alias already exists."),
                Some(_) => Ok(url)
            },
            Err(e) => {
                println!("Err e {}", e);
                Err("Error occurred")
            }
        };

        result
    }

    fn get_urls_collection(&self) -> mongodb::coll::Collection {
        let client = dal::get_mongo_client();
        let coll = client.db("urlshortener").collection("urls");
        coll
    }
}
