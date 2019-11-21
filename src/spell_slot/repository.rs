#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;

use crate::schema::spell_slot;

use super::SpellSlot;

pub fn save(spell_slot: &SpellSlot, connection: &MysqlConnection) -> QueryResult<SpellSlot> {
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

pub fn find_by_id(id: i32, connection: &MysqlConnection) -> QueryResult<SpellSlot> {
    spell_slot::table.find(id)
        .first(connection)
}

pub fn find_by_character_id(id: i32, connection: &MysqlConnection) -> QueryResult<Vec<SpellSlot>> {
    spell_slot::table.filter(spell_slot::character_id.eq(id))
        .load::<SpellSlot>(connection)
}

pub fn update(spell_slot: &SpellSlot, connection: &MysqlConnection) -> QueryResult<usize> {
    diesel::update(spell_slot)
        .set(spell_slot)
        .execute(connection)
}

pub fn delete(id: i32, connection: &MysqlConnection) -> QueryResult<usize> {
    diesel::delete(spell_slot::table.find(id))
        .execute(connection)
}