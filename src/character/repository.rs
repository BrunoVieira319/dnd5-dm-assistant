#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;

use crate::character::CharacterNotes;
use crate::schema::character;

use super::Character;

pub fn save(character: Character, connection: &MysqlConnection) -> QueryResult<Character> {
    match diesel::insert_into(character::table)
        .values(character)
        .execute(connection) {
        Ok(_rows) => fetch_last_inserted(connection),
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

pub fn find_character_notes(id: i32, connection: &MysqlConnection) -> QueryResult<CharacterNotes> {
    let character = find_by_id(id, connection)?;
    let skills = crate::skill::repository::find_by_character_id(character.id.unwrap(), connection)?;
    Ok(Character::notes(character.hit_dice, skills))
}

pub fn update(character: &Character, connection: &MysqlConnection) -> QueryResult<usize> {
    diesel::update(character)
        .set(character)
        .execute(connection)
}