use rocket::serde::json::{json, Json, Value};
use api::db::DbConnection;
use api::db::models::brainlog::{NewBrainlogEntryType, BrainlogEntryType};
use diesel::associations::HasTable;

use diesel::prelude::*;

#[rocket::get("/")]
async fn index(conn: DbConnection) -> Value {
    use diesel::dsl::count_star;
    use api::db::schema::brainlog_entry_type::dsl::*;

    let rows: i64 = conn.run(|c|{
        brainlog_entry_type
        .select(count_star())
        .first(c)
        .expect("Issue")
    }).await;

    let items = conn.run(|c|{
        brainlog_entry_type
        .limit(5)
        .offset(0)
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
    use api::db::schema::brainlog_entry_type;

    let entry = conn.run(|c|{
        diesel::insert_into(brainlog_entry_type::table)
        .values(&NewBrainlogEntryType { name: "x", description: "y" })
        .returning(BrainlogEntryType::as_returning())
        .get_result(c)
        .expect("Error creating")
    }).await;

    Json(entry)
}

#[rocket::launch]
fn rocket() -> _ {
    let figment = rocket::Config::figment().merge((
        "databases.diesel_db.url",
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
    ));

    rocket::custom(figment).attach(DbConnection::fairing()).mount("/", rocket::routes![index, create])
}
