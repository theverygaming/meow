#[macro_export]
macro_rules! crud_create {
    ($route:expr, $fn_name:ident, $table:ident, $t_input:ty, $t_output:ty) => {
        #[rocket::post($route, format = "json", data = "<data>")]
        async fn $fn_name(
            conn: DbConnection,
            _key: ApiKey<'_>,
            data: Json<$t_input>,
        ) -> Json<$t_output> {
            use crate::db::schema::$table;

            let entry = conn
                .run(move |c| {
                    diesel::insert_into($table::table)
                        .values(&*data)
                        .returning(<$t_output>::as_returning())
                        .get_result(c)
                        .expect("Error creating")
                })
                .await;

            Json(entry)
        }
    };
}

#[macro_export]
macro_rules! crud_list {
    ($route:expr, $fn_name:ident, $table:ident, $t_output:ty, $order_by:expr) => {
        #[rocket::get($route)]
        async fn $fn_name(
            conn: DbConnection,
            _key: ApiKey<'_>,
            page: i64,
            mut pagesize: i64
        ) -> Value {
            use crate::db::schema::$table;
            use crate::db::schema::$table::dsl::*;
            use diesel::dsl::count_star;

            let rows: i64 = conn
                .run(|c| $table.select(count_star()).first(c).expect("Issue"))
                .await;

            if pagesize == -1 {
                pagesize = rows;
            }

            let items = conn
                .run(move |c| {
                    $table
                        .limit(pagesize)
                        .offset(page * pagesize)
                        .select(<$t_output>::as_select())
                        .order_by($order_by)
                        .load(c)
                        .expect("Issue")
                })
                .await;

            json!({
                "total_items": rows,
                "items": items,
            })
        }
    };
}

#[macro_export]
macro_rules! crud_update {
    ($route:expr, $fn_name:ident, $table:ident, $t_input:ty, $t_output:ty) => {
        #[rocket::post($route, format = "json", data = "<data>")]
        async fn $fn_name(
            conn: DbConnection,
            _key: ApiKey<'_>,
            id: &str,
            data: Json<$t_input>,
        ) -> Json<$t_output> {
            use crate::db::schema::$table;

            let uuid = uuid::Uuid::parse_str(id).expect("valid UUID");

            let entry = conn
                .run(move |c| {
                    diesel::update($table::table)
                        .filter($table::id.eq(uuid))
                        .set(&*data)
                        .returning(<$t_output>::as_returning())
                        .get_result(c)
                        .expect("Error updating")
                })
                .await;

            Json(entry)
        }
    };
}

#[macro_export]
macro_rules! crud_delete {
    ($route:expr, $fn_name:ident, $table:ident) => {
        #[rocket::get($route)]
        async fn $fn_name(conn: DbConnection, _key: ApiKey<'_>, id: &str) -> () {
            use crate::db::schema::$table;

            let uuid = uuid::Uuid::parse_str(id).expect("valid UUID");

            conn.run(move |c| {
                diesel::delete($table::table)
                    .filter($table::id.eq(uuid))
                    .execute(c)
                    .expect("Error deleting")
            })
            .await;
        }
    };
}
