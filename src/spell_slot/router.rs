use rocket;
use rocket::Route;

use crate::spell_slot::handler::*;

pub fn get_routes() -> (String, Vec<Route>) {
    let routes = routes![
        save
    ];
    (String::from("/spell_slots"), routes)
}