#[macro_use]
extern crate rocket;

use dashmap::DashMap;
use rand::{Rng, thread_rng};
use rocket::response::Redirect;
use rocket::response::status::{BadRequest, NotFound};
use rocket::State;

type RocketState = State<DashMap<u32, String>>;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/api/<key>")]
fn redirect(key: u32, state: &RocketState) -> Result<Redirect, NotFound<&str>> {
    state.get(&key).map(|url| Redirect::to(url.clone())).ok_or(NotFound("Invalid or expired link!"))
}

#[post("/api/shorten?<url>")]
fn shorten(url: String, state: &RocketState) -> Result<String, BadRequest<&str>> {
    if url.is_empty() {
        Err(BadRequest(Some("URL is empty!")))
    } else {
        let key: u32 = thread_rng().gen();

        state.insert(key, url);
        Ok(key.to_string())
    }
}

//noinspection RsMainFunctionNotFound
#[launch]
fn rocket() -> _ {
    rocket::build().manage(DashMap::<u32, String>::new())
        .mount("/", routes![index, redirect, shorten])
}
