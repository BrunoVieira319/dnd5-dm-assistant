#![allow(proc_macro_derive_resolution_fallback)]

use super::schema::skill;
use super::character::Character;

pub mod handler;
pub mod router;
pub mod repository;

pub enum Recover {
    Short,
    Long,
    Unlimited,
}

#[derive(Serialize, Deserialize, Insertable, Queryable, AsChangeset, Identifiable, Associations)]
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
