use diesel::result::Error;
use rocket::http::Status;
use rocket_contrib::json::Json;

use crate::character;
use crate::character::{Character, CharacterNotes, CharacterToSave};
use crate::character::repository::find_character_notes;
use crate::connection::Connection;

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    }
}

#[post("/", format = "application/json", data = "<character>")]
pub fn save(character: Json<CharacterToSave>, connection: Connection) -> Result<Json<Character>, Status> {
    character::repository::save(Character::from_character_to_save(character.into_inner()), &connection)
        .map(|character| Json(character))
        .map_err(|err| error_status(err))
}

#[get("/")]
pub fn find_all(connection: Connection) -> Result<Json<Vec<Character>>, Status> {
    character::repository::find_all(&connection)
        .map(|character| Json(character))
        .map_err(|err| error_status(err))
}

#[get("/<id>")]
pub fn find_by_id(id: i32, connection: Connection) -> Result<Json<Character>, Status> {
    character::repository::find_by_id(id, &connection)
        .map(|character| Json(character))
        .map_err(|err| error_status(err))
}

#[put("/<id>/hp/<expr>")]
pub fn change_hp(id: i32, expr: String, connection: Connection) -> Result<Json<Character>, Status> {
    match character::repository::find_by_id(id, &connection) {
        Ok(mut character) => {
            character.change_hp(&expr);
            character::repository::update(&character, &connection)
                .map(|_rows| Json(character))
                .map_err(|err| error_status(err))
        }
        Err(err) => Result::Err(error_status(err))
    }
}

#[get("/<id>/notes")]
pub fn get_character_notes(id: i32, connection: Connection) -> Result<Json<CharacterNotes>, Status> {
    find_character_notes(id, &connection)
        .map(|notes| Json(notes))
        .map_err(|err| error_status(err))
}