use rocket;
use rocket::Route;

use crate::connection;
use crate::skill::handler::*;

pub fn get_routes() -> (String, Vec<Route>) {
    let routes = routes![save, find_all, find_by_id, find_by_character_id];
    (String::from("/skills"), routes)
}