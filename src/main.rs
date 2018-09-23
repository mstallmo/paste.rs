#![feature(plugin, decl_macro, proc_macro_non_items)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate diesel;
extern crate rocket;

#[cfg(test)]
mod tests;

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

