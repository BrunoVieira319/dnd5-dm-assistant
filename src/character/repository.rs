#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;

use crate::schema::character;

use super::Character;

pub fn save(character: Character, conn: &MysqlConnection) -> Character {
    diesel::insert_into(character::table)
        .values(character)
        .execute(conn)
        .expect("Failed on saving character");

    character::table.order(character::id.desc())
        .first(conn)
        .unwrap()
}

pub fn find_all(conn: &MysqlConnection) -> Vec<Character> {
    character::table.order(character::id.asc())
        .load::<Character>(conn)
        .unwrap()
}

pub fn find_by_id(id: i32, conn: &MysqlConnection) -> Character {
//    character::table.find(id)
}