extern crate urlshortener_core;
extern crate urlshortener_codec;
extern crate urlshortener_data;

mod services;

pub use self::services::url_manager::UrlManager;
pub use urlshortener_core::models;
pub use urlshortener_core::errors;