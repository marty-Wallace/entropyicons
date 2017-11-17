#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate rust_360;
use rust_360::server;

fn main() {
    server::rocket()
        .launch();
}
