#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]
#![feature(custom_attribute)]

#[macro_use(bson, doc)]
extern crate bson;
extern crate mongodb;

extern crate serde;
extern crate serde_json;

pub mod models;
pub mod dal;
