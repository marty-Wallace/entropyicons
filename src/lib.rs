
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate dotenv;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate rocket;
extern crate rocket_contrib;
extern crate rand;
extern crate image;
extern crate serde_derive;
#[macro_use]
extern crate serde_json;


pub mod schema;
pub mod models;
pub mod image_generators;
pub mod server;
pub mod db;
