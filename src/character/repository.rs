#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;
use crate::schema::character;
use super::Character;

pub fn save(character: Character, connection: &MysqlConnection) -> QueryResult<Character> {
    match diesel::insert_into(character::table)
        .values(character)
        .execute(connection) {
        Ok(rows) => fetch_last_inserted(connection),
        Err(err) => return Err(err)
    }
}

fn fetch_last_inserted(connection: &MysqlConnection) -> QueryResult<Character> {
    character::table.order(character::id.desc())
        .first(connection)
}

pub fn find_all(connection: &MysqlConnection) -> QueryResult<Vec<Character>> {
    character::table.order(character::id.asc())
        .load::<Character>(connection)
}

pub fn find_by_id(id: i32, connection: &MysqlConnection) -> QueryResult<Character> {
    character::table.find(id)
        .first(connection)
}