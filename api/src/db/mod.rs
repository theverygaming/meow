pub mod schema;
pub mod models;
pub mod utcdatetime;

use rocket_sync_db_pools::{database, diesel};

#[database("diesel_db")]
pub struct DbConnection(diesel::PgConnection);
