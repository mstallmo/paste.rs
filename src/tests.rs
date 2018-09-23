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