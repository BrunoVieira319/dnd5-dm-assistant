use diesel::result::Error;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

use crate::character;
use crate::character::{Character, CharacterToSave};
use crate::connection::Connection;

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    }
}

#[get("/")]
pub fn find_all(connection: Connection) -> Json<Vec<Character>> {
    Json(character::repository::find_all(&connection))
}

#[post("/", format = "application/json", data = "<character>")]
pub fn save(character: Json<CharacterToSave>, connection: Connection) -> Json<Character> {
    Json(character::repository::save(Character::from_character_to_save(character.into_inner()), &connection))
}