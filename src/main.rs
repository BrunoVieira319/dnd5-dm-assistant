#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
extern crate rocket_cors;
#[macro_use]
extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

mod schema;
mod connection;
mod character;
mod skill;
mod spell_slot;

fn main() {
    let (skill_endpoint, skill_routes) = skill::router::get_routes();
    let (character_endpoint, character_routes) = character::router::get_routes();
    let (ss_endpoint, ss_routes) = spell_slot::router::get_routes();

    let cors = rocket_cors::CorsOptions::default().to_cors().unwrap();

    rocket::ignite()
        .manage(connection::connect())
        .attach(cors)
        .mount(&skill_endpoint, skill_routes)
        .mount(&character_endpoint, character_routes)
        .mount(&ss_endpoint, ss_routes)
        .launch();
}
