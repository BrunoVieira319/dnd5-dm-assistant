#![allow(proc_macro_derive_resolution_fallback)]

use super::character::Character;
use super::schema::skill;

pub mod handler;
pub mod router;
pub mod repository;

pub enum Recover {
    Short,
    Long,
    Unlimited,
}

#[derive(Serialize, Deserialize, Insertable, Queryable, AsChangeset, Identifiable, Associations)]
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

impl Skill {
    pub fn use_skill(&mut self) {
        if self.current_uses > 0 {
            self.current_uses -= 1;
        };
    }

    pub fn recover_uses(&mut self) {
        self.current_uses = self.max_uses
    }
}
