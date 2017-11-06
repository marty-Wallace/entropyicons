#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rand;

use rand::{Rng, SeedableRng, StdRng};
use rocket::request::Request;

#[get("/")]
fn index() -> &'static str {
    "Hello World!"
}

#[get("/<itemset>/<entropy>")]
fn image(itemset: String, entropy: String) -> String {

    // Use the string as a seed for our RNG.
    let seed = entropy
        .as_bytes()
        .iter()
        .map(|&x| x as usize)
        .collect::<Vec<usize>>();

    // init rng
    let mut rng = StdRng::from_seed(seed.as_slice());

    // sample number, poc
    let n : i64 = rng.gen::<i64>();

    format!("Itemset: {} , Entropy: {}, First i64: {}",
            itemset.as_str(),
            entropy.as_str(),
            n
    )
}

// 404 handler
#[error(404)]
fn not_found(_req: &Request) -> &'static str{
    "404 Not Found"
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/api", routes![image])
        .catch(errors![not_found])
        .launch();

}

