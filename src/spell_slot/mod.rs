#![allow(proc_macro_derive_resolution_fallback)]

use super::schema::spell_slot;

pub mod repository;
pub mod handler;
pub mod router;

#[derive(Serialize, Deserialize, Insertable, Queryable, AsChangeset, Identifiable, Associations)]
#[table_name = "spell_slot"]
pub struct SpellSlot {
    pub id: Option<i32>,
    pub character_id: i32,
    pub level: i32,
    pub current_slots: i32,
    pub max_slots: i32,
}