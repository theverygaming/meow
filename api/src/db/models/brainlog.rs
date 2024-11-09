use diesel::prelude::*;
use crate::db::schema::{brainlog_entry_type, brainlog_entry};
use rocket::serde::{Serialize, Deserialize};
use crate::db::utcdatetime::UTCDateTime;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = brainlog_entry_type)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct BrainlogEntryType {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: String,
}

#[derive(Insertable, Deserialize, AsChangeset)]
#[diesel(table_name = brainlog_entry_type)]
pub struct NewBrainlogEntryType {
    pub name: String,
    pub description: String,
}

#[derive(Queryable, Selectable, Associations, Serialize)]
#[diesel(table_name = brainlog_entry)]
#[diesel(belongs_to(BrainlogEntryType, foreign_key = type_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct BrainlogEntry {
    pub id: uuid::Uuid,
    pub time: UTCDateTime,
    pub type_id: uuid::Uuid,
    pub body: String,
}

#[derive(Insertable, Deserialize, AsChangeset)]
#[diesel(table_name = brainlog_entry)]
#[diesel(belongs_to(BrainlogEntryType, foreign_key = type_id))]
pub struct NewBrainlogEntry {
    pub time: UTCDateTime,
    pub type_id: uuid::Uuid,
    pub body: String,
}
