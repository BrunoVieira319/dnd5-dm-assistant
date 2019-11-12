#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;
use crate::schema::skill;
use super::Skill;

pub fn save(skill: Skill, connection: &MysqlConnection) -> QueryResult<Skill> {
    match diesel::insert_into(skill::table)
        .values(skill)
        .execute(connection) {
        Ok(rows) => fetch_last_inserted(connection),
        Err(err) => Err(err),
    }
}

fn fetch_last_inserted(connection: &MysqlConnection) -> QueryResult<Skill> {
    skill::table.order(skill::id.desc())
        .first(connection)
}

pub fn find_all(connection: &MysqlConnection) -> QueryResult<Vec<Skill>> {
    skill::table.order(skill::id.asc())
        .load::<Skill>(connection)
}

pub fn find_by_id(id: i32, connection: &MysqlConnection) -> QueryResult<Skill> {
    skill::table.find(id)
        .first(connection)
}

pub fn find_by_character_id(id: i32, connection: &MysqlConnection) -> QueryResult<Vec<Skill>> {
    skill::table.filter(skill::character_id.eq(id))
        .load::<Skill>(connection)
}