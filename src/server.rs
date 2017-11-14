
use std::io::Cursor;
use rand::{SeedableRng, StdRng};
use rocket;
use rocket::http::{Status};
use rocket::request::Request;
use rocket::response::{Response};
use images::{itemset_delegater};

#[get("/")]
fn index() -> &'static str {
    "Hello World!"
}

#[get("/<itemset>/<entropy>")]
fn image<'r>(itemset: String, entropy: String) -> Response<'r>{

    // Use the entropy string as a seed for our RNG.
    let seed = entropy
        .as_bytes()
        .iter()
        .map(|&x| x as usize)
        .collect::<Vec<_>>();

    // init rng
    let mut rng = StdRng::from_seed(seed.as_slice());

    Response::build()
        .status(Status::Ok)
        .raw_header("ContentType", "image/png")
        .sized_body(Cursor::new(itemset_delegater(&itemset, &mut rng)))
        .finalize()
}


// 404 handler
#[error(404)]
fn not_found(_req: &Request) -> &'static str{
    "404 Not Found"
}

pub fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/api", routes![image])
        .catch(errors![not_found])
}


#[cfg(test)]
mod tests {
    use super::*;
    use rocket::local::Client;
    use rocket::http::{Status, ContentType};

    #[test]
    pub fn rocket_starts() {
        Client::new(rocket()).expect("Rocket failed to start");
    }

    #[test]
    pub fn rocket_index() {
        let client = Client::new(rocket()).expect("Rocket failed to start");
        let req = client.get("/");
        let response = req.dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::Plain));
    }

    #[test]
    pub fn rocket_404() {
        let client = Client::new(rocket()).expect("Rocket failed to start");
        let req = client.get("/not/a/real/url");
        let response = req.dispatch();
        assert_eq!(response.status(), Status::NotFound);
    }

    #[test]
    pub fn rocket_api() {
        let itemset = "itemset";
        let entropy = "entropy";
        let client = Client::new(rocket()).expect("Rocket failed to start");
        let req = client.get(format!("/api/{}/{}", itemset, entropy));
        let response = req.dispatch();

        println!("{:?}", response);
        println!("{:?}", response.headers());

    }

}