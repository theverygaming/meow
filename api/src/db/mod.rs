pub mod schema;
pub mod models;

use rocket_sync_db_pools::{database, diesel};

#[database("diesel_db")]
pub struct DbConnection(diesel::PgConnection);
