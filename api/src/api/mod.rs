pub mod brainlog;

fn routes() -> Vec<rocket::Route> {
    let mut routes = Vec::new();
    routes.append(&mut brainlog::routes());

    routes
}

use crate::db::DbConnection;

#[rocket::launch]
pub fn rocket() -> _ {
    let figment = rocket::Config::figment().merge((
        "databases.diesel_db.url",
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
    ));

    rocket::custom(figment).attach(DbConnection::fairing()).mount("/", routes())
}
