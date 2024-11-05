// @generated automatically by Diesel CLI.

diesel::table! {
    brainlog_entry (id) {
        id -> Uuid,
        time -> Timestamp,
        type_id -> Uuid,
        body -> Text,
    }
}

diesel::table! {
    brainlog_entry_type (id) {
        id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        description -> Text,
    }
}

diesel::joinable!(brainlog_entry -> brainlog_entry_type (type_id));

diesel::allow_tables_to_appear_in_same_query!(
    brainlog_entry,
    brainlog_entry_type,
);
