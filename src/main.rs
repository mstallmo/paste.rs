#![feature(plugin, decl_macro, proc_macro_non_items)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

use rocket_contrib::static_files::StaticFiles;

mod api;
mod database;

fn main() {
    rocket::ignite()
        .mount("/", StaticFiles::from("build"))
        .mount("/api", routes![api::index, api::upload, api::retrieve])
        .launch();
}
