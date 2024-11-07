extern crate rocket;

use rocket::serde::json::{json, Json, Value};
use crate::db::DbConnection;
use crate::db::models::brainlog::{NewBrainlogEntryType, BrainlogEntryType};

use diesel::prelude::*;

#[rocket::get("/?<page>&<pagesize>")]
async fn index(conn: DbConnection, page: i64, pagesize: i64) -> Value {
    use diesel::dsl::count_star;
    use crate::db::schema::brainlog_entry_type::dsl::*;

    let rows: i64 = conn.run(|c|{
        brainlog_entry_type
        .select(count_star())
        .first(c)
        .expect("Issue")
    }).await;

    let items = conn.run(move |c|{
        brainlog_entry_type
        .limit(pagesize)
        .offset(page * pagesize)
        .select(BrainlogEntryType::as_select())
        .load(c)
        .expect("Issue")
    }).await;

    json!({
        "total_items": rows,
        "items": items,
    })
}

#[rocket::get("/create")]
async fn create(conn: DbConnection) -> Json<BrainlogEntryType> {
    use crate::db::schema::brainlog_entry_type;

    let entry = conn.run(|c|{
        diesel::insert_into(brainlog_entry_type::table)
        .values(&NewBrainlogEntryType { name: "x", description: "y" })
        .returning(BrainlogEntryType::as_returning())
        .get_result(c)
        .expect("Error creating")
    }).await;

    Json(entry)
}

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![
        index,
        create
    ]
}
