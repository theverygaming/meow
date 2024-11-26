use crate::api::apikey::ApiKey;
use crate::db::models::quest::{NewQuest, Quest};
use crate::db::DbConnection;
use rocket::serde::json::{json, Json, Value};

use diesel::prelude::*;

use crate::{crud_create, crud_delete, crud_list, crud_update};

crud_create!("/api/quest/create", quest_create, quest, NewQuest, Quest);

crud_list!(
    "/api/quest/list?<page>&<pagesize>",
    quest_list,
    quest,
    Quest,
    quest::name.desc()
);

crud_update!(
    "/api/quest/<id>",
    quest_update,
    quest,
    NewQuest,
    Quest
);

crud_delete!("/api/quest/<id>", quest_delete, quest);

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![quest_create, quest_list, quest_update, quest_delete]
}
