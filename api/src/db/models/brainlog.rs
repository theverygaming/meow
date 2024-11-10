use diesel::prelude::*;
use crate::db::schema::{brainlog_entry};
use rocket::serde::{Serialize, Deserialize};
use crate::db::utcdatetime::UTCDateTime;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = brainlog_entry)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct BrainlogEntry {
    pub id: uuid::Uuid,
    pub time: UTCDateTime,
    pub log_type: String,
    pub body: String,
}

#[derive(Insertable, Deserialize, AsChangeset)]
#[diesel(table_name = brainlog_entry)]
pub struct NewBrainlogEntry {
    pub time: UTCDateTime,
    pub log_type: String,
    pub body: String,
}
