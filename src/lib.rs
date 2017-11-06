
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate dotenv;
#[macro_use] extern crate diesel;
#[macro_use]extern crate diesel_codegen;
extern crate rocket;
extern crate rand;


pub mod schema;
pub mod models;
pub mod images;
pub mod server;
pub mod db;

