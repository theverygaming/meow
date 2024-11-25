pub mod brainlog;
pub mod apikey;

fn routes() -> Vec<rocket::Route> {
    let mut routes = Vec::new();
    routes.append(&mut brainlog::routes());

    routes
}

use crate::db::DbConnection;

const DB_MIGRATIONS: diesel_migrations::EmbeddedMigrations = diesel_migrations::embed_migrations!("./migrations");

#[rocket::launch]
pub fn rocket() -> _ {
    let figment = rocket::Config::figment().merge((
        "databases.diesel_db.url",
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
    ));

    let mut r = rocket::custom(figment)
        .attach(DbConnection::fairing())
        .attach(rocket::fairing::AdHoc::on_liftoff("Run migrations", |rocket| {
            Box::pin(async move {
                use diesel_migrations::{MigrationHarness};
                let conn = DbConnection::get_one(rocket).await.unwrap();
                conn.run(move |c|{
                    println!("running migrations...");
                    c.run_pending_migrations(DB_MIGRATIONS).unwrap();
                }).await;
            })
        }));
    r = r.mount("/", routes());
    match std::env::var("STATIC_DIR") {
        Ok(var) => {
            println!("Serving static files from {}", var);
            r = r.mount("/", rocket::fs::FileServer::from(var).rank(100));
        },
        Err(_) => {},
    };
    r
}
