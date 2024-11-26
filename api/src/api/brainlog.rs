use crate::api::apikey::ApiKey;
use crate::db::models::brainlog::{BrainlogEntry, NewBrainlogEntry};
use crate::db::DbConnection;
use rocket::serde::json::{json, Json, Value};

use diesel::prelude::*;

#[rocket::post("/api/brainlog/create", format = "json", data = "<data>")]
async fn log_create(
    conn: DbConnection,
    _key: ApiKey<'_>,
    data: Json<NewBrainlogEntry>,
) -> Json<BrainlogEntry> {
    use crate::db::schema::brainlog_entry;

    let entry = conn
        .run(move |c| {
            diesel::insert_into(brainlog_entry::table)
                .values(&*data)
                .returning(BrainlogEntry::as_returning())
                .get_result(c)
                .expect("Error creating")
        })
        .await;

    Json(entry)
}

#[rocket::get("/api/brainlog/get?<id>")]
async fn log_get(conn: DbConnection, _key: ApiKey<'_>, id: &str) -> Json<BrainlogEntry> {
    use crate::db::schema::brainlog_entry;

    let uuid = uuid::Uuid::parse_str(id).expect("valid UUID");

    let entry = conn
        .run(move |c| {
            brainlog_entry::table
                .filter(brainlog_entry::id.eq(uuid))
                .first(c)
                .expect("Issue")
        })
        .await;

    Json(entry)
}

#[rocket::post("/api/brainlog/update?<id>", format = "json", data = "<data>")]
async fn log_update(
    conn: DbConnection,
    _key: ApiKey<'_>,
    id: &str,
    data: Json<NewBrainlogEntry>,
) -> Json<BrainlogEntry> {
    use crate::db::schema::brainlog_entry;

    let uuid = uuid::Uuid::parse_str(id).expect("valid UUID");

    let entry = conn
        .run(move |c| {
            diesel::update(brainlog_entry::table)
                .filter(brainlog_entry::id.eq(uuid))
                .set(&*data)
                .returning(BrainlogEntry::as_returning())
                .get_result(c)
                .expect("Error updating")
        })
        .await;

    Json(entry)
}

#[rocket::get("/api/brainlog/delete?<id>")]
async fn log_delete(conn: DbConnection, _key: ApiKey<'_>, id: &str) {
    use crate::db::schema::brainlog_entry;

    let uuid = uuid::Uuid::parse_str(id).expect("valid UUID");

    conn.run(move |c| {
        diesel::delete(brainlog_entry::table)
            .filter(brainlog_entry::id.eq(uuid))
            .execute(c)
            .expect("Error deleting")
    })
    .await;
}

#[rocket::get("/api/brainlog/list?<page>&<pagesize>")]
async fn log_list(conn: DbConnection, _key: ApiKey<'_>, page: i64, mut pagesize: i64) -> Value {
    use crate::db::schema::brainlog_entry;
    use crate::db::schema::brainlog_entry::dsl::*;
    use diesel::dsl::count_star;

    let rows: i64 = conn
        .run(|c| brainlog_entry.select(count_star()).first(c).expect("Issue"))
        .await;

    if pagesize == -1 {
        pagesize = rows;
    }

    let items = conn
        .run(move |c| {
            brainlog_entry
                .limit(pagesize)
                .offset(page * pagesize)
                .select(BrainlogEntry::as_select())
                .order_by(brainlog_entry::time.desc())
                .load(c)
                .expect("Issue")
        })
        .await;

    json!({
        "total_items": rows,
        "items": items,
    })
}

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![log_create, log_get, log_update, log_delete, log_list]
}
