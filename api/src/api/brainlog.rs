use rocket::serde::json::{json, Json, Value};
use crate::db::DbConnection;
use crate::db::models::brainlog::{NewBrainlogEntryType, BrainlogEntryType};

use diesel::prelude::*;

#[rocket::post("/brainlog/create", format = "application/json", data = "<data>")]
async fn create(conn: DbConnection, data: &str) -> Json<BrainlogEntryType> {
    use crate::db::schema::brainlog_entry_type;

    let new_entry: NewBrainlogEntryType = rocket::serde::json::serde_json::from_str(&data).unwrap();

    let entry = conn.run(move |c|{
        diesel::insert_into(brainlog_entry_type::table)
        .values(&new_entry)
        .returning(BrainlogEntryType::as_returning())
        .get_result(c)
        .expect("Error creating")
    }).await;

    Json(entry)
}

#[rocket::get("/brainlog/get?<id>")]
async fn get(conn: DbConnection, id: &str) -> Json<BrainlogEntryType> {
    use crate::db::schema::brainlog_entry_type;

    let uuid = uuid::Uuid::parse_str(id).expect("valid UUID");

    let entry = conn.run(move |c|{
        brainlog_entry_type::table
        .filter(brainlog_entry_type::id.eq(uuid))
        .first(c)
        .expect("Issue")
    }).await;

    Json(entry)
}

#[rocket::get("/brainlog/list?<page>&<pagesize>")]
async fn list(conn: DbConnection, page: i64, pagesize: i64) -> Value {
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

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![
        create,
        get,
        list
    ]
}

#[cfg(test)]
mod test {
    use rocket::local::blocking::Client;
    use rocket::http::Status;

    #[test]
    fn create() {
        let client = Client::tracked(crate::api::rocket()).expect("valid rocket instance");
        let response = client.post("/brainlog/create").body("field=value&otherField=123").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
    
    #[test]
    fn list() {
        let client = Client::tracked(crate::api::rocket()).expect("valid rocket instance");
        let response = client.get("/brainlog/list?page=0&pagesize=5").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
}
