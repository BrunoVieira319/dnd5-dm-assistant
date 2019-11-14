#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;
use crate::schema::spell_slot;
use super::SpellSlot;

pub fn save(spell_slot: SpellSlot, connection: &MysqlConnection) -> QueryResult<SpellSlot> {
    match diesel::insert_into(spell_slot::table)
        .values(spell_slot)
        .execute(connection) {
        Ok(_rows) => fetch_last_inserted(connection),
        Err(err) => Err(err),
    }
}

fn fetch_last_inserted(connection: &MysqlConnection) -> QueryResult<SpellSlot> {
    spell_slot::table.order(spell_slot::id.desc())
        .first(connection)
}