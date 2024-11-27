use crate::api::apikey::ApiKey;
use crate::db::models::quest::{NewQuest, NewQuestItem, Quest, QuestItem};
use crate::db::DbConnection;
use rocket::serde::json::{json, Json, Value};

use diesel::prelude::*;

use crate::{crud_create, crud_delete, crud_list, crud_update};

crud_create!("/api/quest/create", quest_create, quest, NewQuest, Quest);

crud_list!(
    "/api/quest?<page>&<pagesize>",
    quest_list,
    quest,
    Quest,
    quest::name.desc()
);

crud_update!("/api/quest/<id>", quest_update, quest, NewQuest, Quest);

crud_delete!("/api/quest/<id>", quest_delete, quest);

crud_create!(
    "/api/quest/items/create",
    quest_item_create,
    quest_item,
    NewQuestItem,
    QuestItem
);

crud_list!(
    "/api/quest/items?<page>&<pagesize>",
    quest_item_list,
    quest_item,
    QuestItem,
    quest_item::name.desc()
);

crud_update!(
    "/api/quest/items/<id>",
    quest_item_update,
    quest_item,
    NewQuestItem,
    QuestItem
);

crud_delete!("/api/quest/items/<id>", quest_item_delete, quest_item);

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![
        quest_create,
        quest_list,
        quest_update,
        quest_delete,
        quest_item_create,
        quest_item_list,
        quest_item_update,
        quest_item_delete
    ]
}
