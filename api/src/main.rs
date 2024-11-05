use rocket::serde::json::Json;
use api::db::DbConnection;
use api::db::models::brainlog::{NewBrainlogEntryType, BrainlogEntryType};
use diesel::associations::HasTable;

use diesel::prelude::*;

#[rocket::get("/")]
async fn index(conn: DbConnection) -> Json<Vec<BrainlogEntryType>> {
    use api::db::schema::brainlog_entry_type::dsl::*;
    let types = conn.run(move |c| brainlog_entry_type
        .filter(name.eq("a"))
        .limit(5)
        .select(BrainlogEntryType::as_select())
        .load(c)
        .expect("Issue")).await;
    Json(types)
}

#[rocket::get("/create")]
async fn create(conn: DbConnection) -> Json<BrainlogEntryType> {
    use api::db::schema::brainlog_entry_type;
    
    let new = NewBrainlogEntryType { name: "x", description: "y" };
    // FIXME: this somehow doesn't fucking work????
    let entry = conn.run(move |c| 
        diesel::insert_into(brainlog_entry_type::table)
        .values(&new)
        .returning(BrainlogEntryType::as_returning())
        .get_result(c)
        .expect("Error creating")).await;

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
