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
