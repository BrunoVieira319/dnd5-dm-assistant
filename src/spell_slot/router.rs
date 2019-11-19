use rocket;
use rocket::Route;

use crate::spell_slot::handler::*;

pub fn get_routes() -> (String, Vec<Route>) {
    let routes = routes![
        save,
        find_by_id,
        delete_by_id,
        cast_spell,
        recover_slot
    ];
    (String::from("/spell_slots"), routes)
}