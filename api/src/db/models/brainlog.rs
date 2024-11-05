use diesel::prelude::*;
use crate::db::schema::{brainlog_entry_type, brainlog_entry};
use serde::{Serialize};


#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = brainlog_entry_type)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct BrainlogEntryType {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: String,
}

#[derive(Insertable)]
#[diesel(table_name = brainlog_entry_type)]
pub struct NewBrainlogEntryType<'a> {
    pub name: &'a str,
    pub description: &'a str,
}

#[derive(Queryable, Selectable, Associations)]
#[diesel(table_name = brainlog_entry)]
#[diesel(belongs_to(BrainlogEntryType, foreign_key = type_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct BrainlogEntry {
    pub id: uuid::Uuid,
    pub time: chrono::NaiveDateTime,
    pub type_id: uuid::Uuid,
    pub body: String,
}
