#![allow(proc_macro_derive_resolution_fallback)]
use super::schema::skill;
use diesel;
use super::character::Character;

pub enum Recover {
    Short,
    Long,
    Unlimited,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Character)]
#[table_name = "skill"]
pub struct Skill {
    pub id: Option<i32>,
    pub character_id: i32,
    pub name: String,
    pub description: String,
    pub max_uses: i32,
    pub current_uses: i32,
    pub recover: String,
}
