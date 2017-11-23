
extern crate entropyicons;

extern crate rocket;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use rocket::local::Client;
use rocket::http::{Status, ContentType};
use serde_json::from_str;

#[derive(Serialize, Deserialize)]
struct List {
    image_sets: Vec<String>,
}

#[test]
fn all_listed_imageset_working() {

    let client = Client::new(entropyicons::server::dev_app())
        .expect("Failed to start Rocket");
    let req = client.get("/api/list");
    let mut response = req.dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let body = response.body_string()
        .expect("Failed to load /api/list response body");

    let list: List = from_str(&body)
        .expect("Failed to parse JSON from /api/list response body");


    let name = "Boxi@icons.mvw.io";
    for imageset in list.image_sets {
        let req = client.get(format!("/api/{}/{}", imageset.as_str(), name));
        let resp = req.dispatch();
        assert_eq!(resp.status(), Status::Ok);
    }

}