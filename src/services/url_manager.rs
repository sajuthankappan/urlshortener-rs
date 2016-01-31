use urlshortener_core::models;
use urlshortener_core::errors;
use urlshortener_data::repository::UrlRepository;

pub struct UrlManager;

impl UrlManager {
    pub fn new() -> UrlManager {
        UrlManager
    }

    pub fn find_one(&self, alias: String) -> Option<models::Url> {
        UrlRepository::new().find_one(alias)
    }

    pub fn find_by_long_url(&self, long_url: String) -> Option<models::Url> {
        UrlRepository::new().find_by_long_url(long_url)
    }

    pub fn add(&self, url: models::Url) -> Result<models::Url, errors::UrlError> {
        let url_repository = UrlRepository::new(); 
        let insert_result = url_repository.add(url);
        match insert_result {
            Ok(v) => return Ok(v),
            Err(e) => {
                return Err(e);
            }
        };
	}
}
