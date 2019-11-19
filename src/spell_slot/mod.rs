#![allow(proc_macro_derive_resolution_fallback)]

use super::schema::spell_slot;

pub mod repository;
pub mod handler;
pub mod router;

#[derive(Serialize, Deserialize)]
pub struct SpellSlotToSave {
    pub character_id: i32,
    pub level: i32,
    pub max_slots: i32,
}

#[derive(Serialize, Deserialize, Insertable, Queryable, AsChangeset, Identifiable, Associations)]
#[table_name = "spell_slot"]
pub struct SpellSlot {
    pub id: Option<i32>,
    pub character_id: i32,
    pub level: i32,
    pub current_slots: i32,
    pub max_slots: i32,
}

impl SpellSlot {
    fn from_spell_slot_to_save(spell_slot: SpellSlotToSave) -> SpellSlot {
        SpellSlot {
            id: None,
            character_id: spell_slot.character_id,
            level: spell_slot.level,
            current_slots: spell_slot.max_slots,
            max_slots: spell_slot.max_slots,
        }
    }

    fn cast(&mut self) {
        if self.current_slots > 0 {
            self.current_slots -= 1
        }
    }

    fn recover(&mut self) {
        if self.current_slots < self.max_slots {
            self.current_slots += 1
        }
    }
}