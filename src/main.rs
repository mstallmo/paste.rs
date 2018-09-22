#![feature(plugin, decl_macro, proc_macro_non_items)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;

#[macro_use]
mod macros;
mod api;
mod database;
mod app;

fn main() {
    rocket::ignite()
        .mount("/", routes![app::index, app::files, app::catch_unknown_routes])
        .mount("/api", routes![api::index, api::upload, api::retrieve])
        .launch();
}
