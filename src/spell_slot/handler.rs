use diesel::result::Error;
use rocket::http::Status;
use rocket_contrib::json::{Json, JsonValue};

use crate::connection::Connection;
use crate::spell_slot;
use crate::spell_slot::SpellSlotToSave;

use super::SpellSlot;

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    }
}

#[post("/", data = "<spell_slot>")]
pub fn save(spell_slot: Json<SpellSlotToSave>, connection: Connection) -> Result<Json<SpellSlot>, Status> {
    spell_slot::repository::save(SpellSlot::from_spell_slot_to_save(spell_slot.into_inner()), &connection)
        .map(|spell_slot| Json(spell_slot))
        .map_err(|err| error_status(err))
}

#[get("/<id>")]
pub fn find_by_id(id: i32, connection: Connection) -> Result<Json<SpellSlot>, Status> {
    spell_slot::repository::find_by_id(id, &connection)
        .map(|spell_slot| Json(spell_slot))
        .map_err(|err| error_status(err))
}

#[delete("/<id>")]
pub fn delete_by_id(id: i32, connection: Connection) -> Result<Json<JsonValue>, Status> {
    spell_slot::repository::delete(id, &connection)
        .map(|rows| Json(json!({"deleted": rows})))
        .map_err(|err| error_status(err))
}

#[put("/<id>/cast")]
pub fn cast_spell(id: i32, connection: Connection) -> Result<Json<SpellSlot>, Status> {
    let spell_slot = spell_slot::repository::find_by_id(id, &connection)
        .map_err(|err| error_status(err))
        .and_then(|mut spell_slot| {
            spell_slot.cast();
            Ok(spell_slot)
        })?;

    spell_slot::repository::update(&spell_slot, &connection)
        .map(|_rows| Json(spell_slot))
        .map_err(|err| error_status(err))
}

#[put("/<id>/recover")]
pub fn recover_slot(id: i32, connection: Connection) -> Result<Json<SpellSlot>, Status> {
    let spell_slot = spell_slot::repository::find_by_id(id, &connection)
        .map_err(|err| error_status(err))
        .and_then(|mut spell_slot| {
            spell_slot.recover();
            Ok(spell_slot)
        })?;

    spell_slot::repository::update(&spell_slot, &connection)
        .map(|_rows| Json(spell_slot))
        .map_err(|err| error_status(err))
}