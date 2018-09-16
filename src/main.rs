#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

use rocket::response::NamedFile;
use rocket::Data;
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};

mod paste_id;
use crate::paste_id::PasteID;

#[get("/")]
fn index() -> &'static str {
    "\
    USAGE

        POST /
            accepts raw data in teh body of the request and responds with a URL of
            a page containing the body's content

        GET /<id>
            retrieves the content for the paste with id `<id>`
    "
}

#[get("/web")]
fn web() -> io::Result<NamedFile> {
    NamedFile::open("web/build/index.html")
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("web/build/").join(file)).ok()
}

#[post("/", data = "<paste>")]
fn upload(paste: Data) -> io::Result<String> {
    let id = PasteID::new(3);
    let filename = format!("upload/{id}", id = id);
    let url = format!("{host}/{id}\n", host = "http://localhost:8000", id = id);

    paste.stream_to_file(Path::new(&filename))?;
    Ok(url)
}

//#[get("/<id>")]
//fn retrieve(id: PasteID) -> Option<File> {
//    let filename = format!("upload/{id}", id = id);
//    File::open(&filename).ok()
//}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, upload, /*retrieve,*/ web, files])
        .launch();
}
