use diesel::result::Error;
use rocket::http::Status;
use rocket_contrib::json::Json;

use crate::skill;
use crate::skill::Skill;
use crate::connection::Connection;

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    }
}

#[post("/", data = "<skill>")]
pub fn save(skill: Json<Skill>, connection: Connection) -> Result<Json<Skill>, Status> {
    skill::repository::save(skill.into_inner(), &connection)
        .map(|skill| Json(skill))
        .map_err(|err| error_status(err))
}

#[get("/")]
pub fn find_all(connection: Connection) -> Result<Json<Vec<Skill>>, Status> {
    skill::repository::find_all(&connection)
        .map(|skill| Json(skill))
        .map_err(|err| error_status(err))
}

#[get("/<id>")]
pub fn find_by_id(id: i32, connection: Connection) -> Result<Json<Skill>, Status> {
    skill::repository::find_by_id(id, &connection)
        .map(|skill| Json(skill))
        .map_err(|err| error_status(err))
}

#[get("/character/<character_id>")]
pub fn find_by_character_id(character_id: i32, connection: Connection) -> Result<Json<Vec<Skill>>, Status> {
    skill::repository::find_by_character_id(character_id, &connection)
        .map(|skill| Json(skill))
        .map_err(|err| error_status(err))
}