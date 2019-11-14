use rocket;
use rocket::Route;

use crate::skill::handler::*;

pub fn get_routes() -> (String, Vec<Route>) {
    let routes = routes![
        save,
        find_all,
        find_by_id,
        find_by_character_id,
        use_skill,
        recover_uses_skill
    ];
    (String::from("/skills"), routes)
}