#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]


extern crate rust_360;
use rust_360::server::rocket;

fn main() {
    rocket().launch();
}
