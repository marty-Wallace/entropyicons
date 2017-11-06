
use rand::{Rng, SeedableRng, StdRng};
use rocket;
use rocket::request::Request;
use rocket::response::{NamedFile};

#[get("/")]
fn index() -> &'static str {
    "Hello World!"
}

#[get("/<itemset>/<entropy>")]
fn image(itemset: String, entropy: String) -> NamedFile {

    // Use the string as a seed for our RNG.
    let seed = entropy
        .as_bytes()
        .iter()
        .map(|&x| x as usize)
        .collect::<Vec<_>>();

    // init rng
    let mut rng = StdRng::from_seed(seed.as_slice());

    // sample number, poc
    let n = rng.gen::<i64>();

    // debug itemset and entropy info
    println!("Itemset: {} , Entropy: {}, First i64: {}",
            itemset.as_str(),
            entropy.as_str(),
            n
    );

    // TODO implement image module to take in a rng and an itemset and return a png
    // TODO impl<'r> Responder<'r> for GenPNG in image module
    // example of returning an image using rocket
    NamedFile::open("image.png").expect("Failed to open image")
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

}