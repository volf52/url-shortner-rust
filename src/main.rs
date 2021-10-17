#[macro_use]
extern crate rocket;

use dashmap::DashMap;
use rand::{Rng, thread_rng};
use rocket::{fs::FileServer, response::{Redirect, status::{BadRequest, NotFound}}, State};

type RocketState = State<DashMap<u32, String>>;

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
        .mount("/", routes![redirect, shorten])
        .mount("/", if cfg!(debug_assertions) {
            FileServer::from(rocket::fs::relative!("/frontend/build"))
        } else {
            FileServer::from("/app/static")
        },
        )
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

        assert_eq!(redirect, "https://duck.com");
    }

    #[test]
    fn empty_url() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.post("/api/shorten?url=").dispatch();

        assert_eq!(response.status(), Status::BadRequest);
    }

    #[test]
    fn invalid_key() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.post("/api/get?key=123").dispatch();

        assert_eq!(response.status(), Status::NotFound);
    }

    fn static_site() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/").dispatch();

        assert_eq!(response.status(), Status::Ok);
    }
}
