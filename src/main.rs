#![feature(plugin, decl_macro, proc_macro_non_items)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate diesel;
extern crate rocket;

#[macro_use]
mod macros;
mod api;
mod database;
mod app;

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![app::index, app::files, app::catch_unknown_routes])
        .mount("/api/v1", routes![api::index, api::upload, api::retrieve])
        .manage(database::init_pool())
}

fn main() {
    rocket().launch();
}

#[cfg(test)]
mod tests {
    use super::rocket;
    use rocket::local::Client;
    use rocket::http::Status;


    #[test]
    fn api_docs() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get("/api/v1/docs").dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("\
    USAGE

        POST api/v1/
            accepts raw data in teh body of the request and responds with a URL of
            a page containing the body's content

        GET api/v1/<id>
            retrieves the content for the paste with id `<id>`
    ".into()));
    }

    #[test]
    fn app() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get("/").dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert!(response.body_string().unwrap().contains("<!doctype html>"));
    }
}