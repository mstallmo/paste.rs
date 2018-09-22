use rocket::response::NamedFile;
use std::io;
use std::path::{Path, PathBuf};


#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("build/index.html")
}

#[get("/static/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("build/static/").join(file)).ok()
}

#[get("/<file..>", rank=2)]
fn catch_unknown_routes(file: PathBuf) -> io::Result<NamedFile> {
    ignore!(file);
    NamedFile::open("build/index.html")
}