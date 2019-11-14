#![allow(proc_macro_derive_resolution_fallback)]

use std::cmp::{max, min};

use super::schema::character;
use crate::skill::Skill;

pub mod handler;
pub mod router;
pub mod repository;

pub enum CharacterClass {
    Barbarian,
    Bard,
    Warlock,
    Cleric,
    Druid,
    Sorcerer,
    Fighter,
    Rogue,
    Wizard,
    Monk,
    Paladin,
    Ranger,
}

#[derive(Serialize, Deserialize)]
pub struct CharacterNotes {
    pub hit_dice: i32,
    pub skills: Vec<Skill>
}

#[derive(Serialize, Deserialize)]
pub struct CharacterToSave {
    pub character_name: String,
    pub character_class: String,
    pub race: String,
    pub image: Option<String>,
    pub level: i32,
    pub max_hp: i32,
}

#[table_name = "character"]
#[derive(Serialize, Deserialize, Insertable, Queryable, AsChangeset, Identifiable, Associations)]
pub struct Character {
    pub id: Option<i32>,
    pub character_name: String,
    pub character_class: String,
    pub race: String,
    pub image: Option<String>,
    pub level: i32,
    pub max_hp: i32,
    pub current_hp: i32,
    pub hit_dice: i32,
}

impl Character {
    fn from_character_to_save(character: CharacterToSave) -> Character {
        Character {
            id: None,
            character_name: character.character_name,
            character_class: character.character_class,
            race: character.race,
            image: character.image,
            level: character.level,
            max_hp: character.max_hp,
            current_hp: character.max_hp,
            hit_dice: character.level,
        }
    }

    fn notes(hit_dice: i32, skills: Vec<Skill>) -> CharacterNotes {
        CharacterNotes {
            hit_dice,
            skills
        }
    }

    fn change_hp(&mut self, expr: &str) {
        let updated_hp = if expr.starts_with("+") {
            let heal = expr[1..].parse::<u32>().unwrap_or(0);
            min(self.current_hp + heal as i32, self.max_hp)
        } else {
            let damage  = expr.parse::<u32>().unwrap_or(0);
            max(self.current_hp - damage as i32, 0)
        };
        self.current_hp = updated_hp;
    }
}