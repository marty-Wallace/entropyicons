#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate entropyicons;

use std::env;
use entropyicons::server;

/// Entry point for binary to start up server. Will detect whether the server is running in
/// production of development/staging and start the app accordingly.
fn main() {
    // check if we're running in production, if we are then
    // we will request the app which doesn't mount static files
    match env::var("ROCKET_ENV") {
        Ok(ref v) if v == "production" => {
            server::app()
        }
        _ => {
            server::dev_app()
        }
    }.launch();
}
