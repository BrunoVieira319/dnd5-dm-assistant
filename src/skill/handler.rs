use diesel::result::Error;
use rocket::http::Status;
use rocket_contrib::json::Json;

use crate::connection::Connection;
use crate::skill;
use crate::skill::Skill;

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
        .map(|skills| Json(skills))
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

#[put("/<id>/use")]
pub fn use_skill(id: i32, connection: Connection) -> Result<Json<Skill>, Status> {
    let skill = skill::repository::find_by_id(id, &connection)
        .map_err(|err| error_status(err))
        .and_then(|mut skill| {
            skill.use_skill();
            Ok(skill)
        })?;

    skill::repository::update(&skill, &connection)
        .map(|_rows| Json(skill))
        .map_err(|err| error_status(err))
}

#[put("/<id>/recover")]
pub fn recover_uses_skill(id: i32, connection: Connection) -> Result<Json<Skill>, Status> {
    let skill = skill::repository::find_by_id(id, &connection)
        .map_err(|err| error_status(err))
        .and_then(|mut skill| {
            skill.recover_uses();
            Ok(skill)
        })?;

    skill::repository::update(&skill, &connection)
        .map(|_rows| Json(skill))
        .map_err(|err| error_status(err))
}