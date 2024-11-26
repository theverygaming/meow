use diesel::prelude::*;
use crate::db::schema::{quest, quest_item};
use rocket::serde::{Serialize, Deserialize};
use rocket::serde::json::serde_json;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = quest)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Quest {
    pub id: uuid::Uuid,
    pub name: String,
}

#[derive(Insertable, Deserialize, AsChangeset)]
#[diesel(table_name = quest)]
pub struct NewQuest {
    pub name: String,
}

#[derive(Queryable, Selectable, Serialize, Associations)]
#[diesel(table_name = quest_item)]
#[diesel(belongs_to(Quest, foreign_key = quest_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct QuestItem {
    pub id: uuid::Uuid,
    pub quest_id: uuid::Uuid,
    pub attributes: serde_json::Value,
    pub name: String,
    pub body: String,
}

#[derive(Insertable, Deserialize, AsChangeset, Associations)]
#[diesel(belongs_to(Quest, foreign_key = quest_id))]
#[diesel(table_name = quest_item)]
pub struct NewQuestItem {
    pub quest_id: uuid::Uuid,
    pub attributes: serde_json::Value,
    pub name: String,
    pub body: String,
}
