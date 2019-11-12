use crate::character::handler::*;
use crate::connection;
use rocket;

pub fn create_routes() {
    rocket::ignite()
        .manage(connection::connect())
        .mount("/characters",
               routes![save, find_all, find_by_id],
        ).launch();
}