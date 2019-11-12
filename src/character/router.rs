use rocket;
use rocket::Route;

use crate::character::handler::*;
use crate::connection;

pub fn get_routes() -> (String, Vec<Route>){
    let routes = routes![save, find_all, find_by_id];
    (String::from("/characters"), routes)
}