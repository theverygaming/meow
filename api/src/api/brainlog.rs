use crate::api::apikey::ApiKey;
use crate::db::models::brainlog::{BrainlogEntry, NewBrainlogEntry};
use crate::db::DbConnection;
use rocket::serde::json::{json, Json, Value, serde_json};

use diesel::prelude::*;

use crate::{crud_create, crud_delete, crud_list, crud_update};

crud_create!(
    "/api/brainlog/create",
    log_create,
    brainlog_entry,
    NewBrainlogEntry,
    BrainlogEntry
);

crud_list!(
    "/api/brainlog?<page>&<pagesize>&<search>",
    log_list,
    brainlog_entry,
    BrainlogEntry,
    brainlog_entry::time.desc()
);

crud_update!(
    "/api/brainlog/<id>",
    log_update,
    brainlog_entry,
    NewBrainlogEntry,
    BrainlogEntry
);

crud_delete!("/api/brainlog/<id>", log_delete, brainlog_entry);

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![log_create, log_list, log_update, log_delete]
}
