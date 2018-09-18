use rocket::response::NamedFile;
use std::io;
use std::path::{Path, PathBuf};

///Serves compiled React application to the user
#[get("/")]
fn web() -> io::Result<NamedFile> {
    NamedFile::open("build/index.html")
}

///Serves associated files not the html for the React application
#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("build/").join(file)).ok()
}
