extern crate urlshortener;

use urlshortener::models;
use urlshortener::dal::repository::{UrlRepository, CounterRepository};

#[test]
fn find_for_qi_returns_some() {
    let url = UrlRepository::new().find_one("qi".to_string()).unwrap();
    assert_eq!("http://google.com", url.long_url);
}

//#[test]
fn add_url_adds_row() {
    let mut url = models::Url {id: Some("test".to_string()), long_url: "test.com".to_string() };
    let added = UrlRepository::new().add(&mut url).unwrap();
    assert_eq!(true, added);
}

//#[test]
fn add_url_with_no_id_adds_row() {
    let mut url = models::Url {id: None, long_url: "test.com".to_string() };
    let added = UrlRepository::new().add(&mut url).unwrap();
    assert_eq!(true, added);
}

#[test]
fn increment_counter_works() {
    let count = CounterRepository::new().increment_counter("Url");
    assert_eq!(true, count > 0);
}
