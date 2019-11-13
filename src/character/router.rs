use rocket;
use rocket::Route;

use crate::character::handler::*;

pub fn get_routes() -> (String, Vec<Route>){
    let routes = routes![save, find_all, find_by_id, change_hp];
    (String::from("/characters"), routes)
}