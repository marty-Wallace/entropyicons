
use std::io::Cursor;
use std::path::{Path, PathBuf};
use rand::{SeedableRng, StdRng};
use rocket;
use rocket::http::Status;
use rocket::request::Request;
use rocket_contrib::Json;
use rocket_contrib::Template;
use rocket::response::{status, Response, NamedFile};
use image_generators::imageset_delegator;
use db::{ establish_connection, active_imagesets};


#[get("/")]
/// Homepage for entropy icons returns an Html Response
///
fn index() -> Template {
    let conn = establish_connection();
    let imageset_results = active_imagesets(&conn);
    let mut imagesets = vec![];
    for image in imageset_results {
        imagesets.push(image.name);
    }

    let ctx = json!({
        "imagesets": imagesets
    });

    Template::render("index", &ctx)

}

#[get("/api/list")]
/// JSON endpoint to return all of the current active image sets
///
fn list() -> Json {
    let conn = establish_connection();
    let im_sets = active_imagesets(&conn);
    let results = im_sets.iter().map(|ref im| im.name.clone()).collect::<Vec<_>>();
    Json(json!({
        "image_sets": results
    }))
}

#[get("/api/<imageset>/<entropy>")]
/// The imageset endpoint, if the imageset exists it returns a png
///
/// # Arguments
///
/// * imageset - A String that indicates which image is used to generate the response
/// * entropy - A String used to seed the RNG object used to generate the response
///
/// # Example
/// https://icons.mvw.io/api/squares/martinwallace
///
fn image<'r>(imageset: String, entropy: String) -> Result<Response<'r>, status::NotFound<Template>> {

    // Use the entropy string as a seed for our RNG.
    let seed = entropy
        .as_bytes()
        .iter()
        .map(|&x| x as usize)
        .collect::<Vec<_>>();

    // init rng
    let mut rng = StdRng::from_seed(seed.as_slice());
    let im = imageset_delegator(&imageset, &mut rng);
    match im {
        Ok(i) => {
            Ok(
                Response::build()
                .status(Status::Ok)
                .raw_header("ContentType", "image/png")
                .raw_header("Cache-control", "max-age=31536000")
                .sized_body(Cursor::new(i))
                .finalize()
            )
        },
        Err(_e) => {
            let ctx = json!({
                "imageset" : imageset,
                "entropy" : entropy,
            });
            Err(status::NotFound(Template::render("404", &ctx)))
        }
    }

}

#[get("/<file..>")]
/// Serves all of the static files, should only be used for dev/staging not for production
///
fn static_files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

// 404 handler
#[error(404)]
/// 404 error handler, renders a simple template with some basic error messages
///
fn not_found(req: &Request) -> Template {
    let ctx = json!({
        "uri": format!("{}", req.uri())
    });
    Template::render("404", &ctx)
}

// 500 handler
#[error(500)]
/// 500 error handler, renders a simple template with some basic error messages
///
fn internal_error(req: &Request) -> Template {
    let ctx = json!({
        "uri": format!("{}", req.uri())
    });
    Template::render("500", &ctx)
}

/// Builds the Rocket app, which just need to have .launch() called on it to start
/// Assumes that the /static files will be handled somewhere else, like nginx
///
pub fn app() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, list, image])
        .attach(Template::fairing())
        .catch(errors![not_found, internal_error])
}

/// Returns the same Rocket app as app() but mounts the static files
/// useful for development and staging
///
pub fn dev_app() -> rocket::Rocket {
    app().mount("/static", routes![static_files])
}


#[cfg(test)]
mod tests {
    use super::*;
    use rocket::local::Client;
    use rocket::http::{Status, ContentType};

    #[test]
    /// Tests that the rocket app starts
    ///
    pub fn rocket_starts() {
        Client::new(app()).expect("Rocket failed to start");
    }

    #[test]
    /// Test that the index page is responding
    ///
    pub fn rocket_index() {
        let client = Client::new(app()).expect("Rocket failed to start");
        let req = client.get("/");
        let response = req.dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::HTML));
    }

    #[test]
    /// Test that 404 is operating correctly
    ///
    pub fn rocket_404() {
        let client = Client::new(app()).expect("Rocket failed to start");
        let req = client.get("/test/a/url/that/is/not/a/real/url");
        let response = req.dispatch();
        assert_eq!(response.status(), Status::NotFound);
        assert_eq!(response.content_type(), Some(ContentType::HTML));
    }

    #[test]
    /// Test that api is returning a response for a valid imageset and a 404 for an invalid one
    ///
    pub fn rocket_api() {
        let client = Client::new(app()).expect("Rocket failed to start");

        // valid
        let imageset = "squares";
        let entropy = "entropy";
        let req = client.get(format!("/api/{}/{}", imageset, entropy));
        let response = req.dispatch();
        assert_eq!(response.status(), Status::Ok);

        // invalid
        let imageset = "not-a-real-imageset";
        let entropy = "entropy";
        let req = client.get(format!("/api/{}/{}", imageset, entropy));
        let response = req.dispatch();
        assert_eq!(response.status(), Status::NotFound);
    }

    #[test]
    /// Check that the api is able to list all of the available imagesets
    ///
    pub fn rocket_list() {
        let client = Client::new(app()).expect("Rocket failed to start");

        let req = client.get("/api/list");
        let response = req.dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::JSON));

    }
}
