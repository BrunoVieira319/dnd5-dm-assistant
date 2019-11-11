#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

mod schema;
mod connection;
mod character;
mod skill;

fn main() {
    character::router::create_routes();
}
