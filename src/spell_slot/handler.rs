use diesel::result::Error;
use rocket::http::Status;
use rocket_contrib::json::Json;

use crate::connection::Connection;
use crate::spell_slot;
use super::SpellSlot;

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    }
}

#[post("/", data = "<spell_slot>")]
pub fn save(spell_slot: Json<SpellSlot>, connection: Connection) -> Result<Json<SpellSlot>, Status> {
    spell_slot::repository::save(spell_slot.into_inner(), &connection)
        .map(|spell_slot| Json(spell_slot))
        .map_err(|err| error_status(err))
}