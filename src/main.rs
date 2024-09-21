#[macro_use] extern crate rocket;
use rocket::serde::json::Json;
use rocket::{Request, Response, fairing::{Fairing, Info, Kind}};

mod marvel_service;

#[get("/comics")]  // route for fetching comics
async fn get_comics() -> Json<Vec<marvel_service::Comic>> {
    match marvel_service::fetch_marvel_comics().await {
        Ok(comics) => Json(comics),
        Err(_) => Json(vec![]),  // empty array on error
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![get_comics])  // get_comics route
        .attach(CORS)  // CORS handler
}

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    // fix lifetime params in on_response to match the trait
    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(rocket::http::Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(rocket::http::Header::new("Access-Control-Allow-Methods", "GET, POST, OPTIONS"));
        response.set_header(rocket::http::Header::new("Access-Control-Allow-Headers", "Content-Type"));
    }
}
