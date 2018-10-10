#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate diesel;
extern crate rocket;
extern crate rocket_igniter;

use rocket_igniter::engine::{CliCommand, Engine};

#[cfg(test)]
mod tests;

#[macro_use]
mod macros;
mod api;
mod app;
mod database;

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount(
            "/",
            routes![app::index, app::files, app::catch_unknown_routes],
        )
        .mount("/api/v1", routes![api::index, api::upload, api::retrieve])
        .manage(database::init_pool())
        .attach(Engine::new(CliCommand::YARN))
}

fn main() {
    rocket().launch();
}
