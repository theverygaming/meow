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
    finance_account (id) {
        id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        #[sql_name = "type"]
        type_ -> Int4,
        initial_balance -> Float8,
        opening_time -> Timestamp,
    }
}

diesel::table! {
    finance_transaction (id) {
        id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        description -> Text,
    }
}

diesel::table! {
    finance_transaction_item (id) {
        id -> Uuid,
        transaction_id -> Uuid,
        description -> Text,
        account_id -> Uuid,
        debit -> Float8,
        credit -> Float8,
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

diesel::joinable!(finance_transaction_item -> finance_account (account_id));
diesel::joinable!(finance_transaction_item -> finance_transaction (transaction_id));
diesel::joinable!(quest_item -> quest (quest_id));

diesel::allow_tables_to_appear_in_same_query!(
    brainlog_entry,
    finance_account,
    finance_transaction,
    finance_transaction_item,
    quest,
    quest_item,
);
