table! {
    character (id) {
        id -> Nullable<Integer>,
        character_name -> Varchar,
        character_class -> Varchar,
        race -> Varchar,
        image -> Nullable<Varchar>,
        level -> Integer,
        max_hp -> Integer,
        current_hp -> Integer,
        hit_dice -> Integer,
    }
}

table! {
    skill (id) {
        id -> Nullable<Integer>,
        character_id -> Integer,
        name -> Varchar,
        description -> Text,
        max_uses -> Integer,
        current_uses -> Integer,
        recover -> Varchar,
    }
}

table! {
    spell (id) {
        id -> Nullable<Integer>,
        character_id -> Integer,
        prepared -> Nullable<Bool>,
        data_url -> Varchar,
    }
}

table! {
    spell_slot (id) {
        id -> Nullable<Integer>,
        character_id -> Integer,
        level -> Integer,
        current_quantity -> Integer,
        max_quantity -> Integer,
    }
}

joinable!(skill -> character (character_id));
joinable!(spell -> character (character_id));
joinable!(spell_slot -> character (character_id));

allow_tables_to_appear_in_same_query!(
    character,
    skill,
    spell,
    spell_slot,
);
