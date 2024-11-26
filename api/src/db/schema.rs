// @generated automatically by Diesel CLI.

diesel::table! {
    brainlog_entry (id) {
        id -> Uuid,
        time -> Timestamp,
        body -> Text,
        #[max_length = 255]
        log_type -> Varchar,
    }
}

diesel::table! {
    quest (id) {
        id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::table! {
    quest_item (id) {
        id -> Uuid,
        quest_id -> Uuid,
        attributes -> Jsonb,
        #[max_length = 255]
        name -> Varchar,
        body -> Text,
    }
}

diesel::joinable!(quest_item -> quest (quest_id));

diesel::allow_tables_to_appear_in_same_query!(
    brainlog_entry,
    quest,
    quest_item,
);
