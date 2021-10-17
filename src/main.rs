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

#[get("/api/get?<key>")]
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

#[cfg(test)]
mod tests {
    use rocket::{http::Status, local::blocking::Client};

    use super::rocket;

    #[test]
    fn simple_demo_test() {
        let x = 1 + 1;
        assert_eq!(x, 2)
    }

    #[test]
    fn valid_requests() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");

        let response = client.post("/api/shorten?url=https://duck.com").dispatch();

        assert_eq!(response.status(), Status::Ok);

        let key: u32 = response.into_string().expect("body").parse().expect("valid u32");

        let response = client.get(format!("/api/get?key={}", key)).dispatch();

        assert_eq!(response.status(), Status::SeeOther);

        let redirect = response.headers().get_one("Location").expect("location header");

        assert_eq!(redirect, "https://duck.com")
    }
}
