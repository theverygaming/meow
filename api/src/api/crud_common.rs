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
            mut pagesize: i64,
            search: Option<&str>,
        ) -> Value {
            use crate::db::schema::$table;
            use crate::db::schema::$table::dsl::*;
            use diesel::dsl::count_star;

            let search_data = match search {
                Some(x) => serde_json::from_str(x).unwrap(),
                None => json!([]),
            };

            type TableFilter = Box<dyn BoxableExpression<$table::table, diesel::pg::Pg, SqlType = diesel::sql_types::Bool>>;

            fn create_filter(search_data: &Value) -> TableFilter {
                let mut filters: Vec<TableFilter> = Vec::new();

                // JSON structure: [["id", "=", "abcdef"], ["name", "=", "meow"]]
                if let Some(operations) = search_data.as_array() {
                    for op in operations {
                        let op_items = match op.as_array() {
                            Some(items) if items.len() == 3 => items,
                            _ => continue, // invalid? skip
                        };

                        let (field, operator, value) = match (
                            op_items[0].as_str(),
                            op_items[1].as_str(),
                            op_items[2].as_str(),
                        ) {
                            (Some(field), Some(operator), Some(value)) => (field, operator, value),
                            _ => continue, // invalid format? skip
                        };

                        let filter = match (field, operator) {
                            ("id", "=") => {
                                if let Ok(uuid) = uuid::Uuid::parse_str(value) {
                                    Some(Box::new($table::id.eq(uuid)))
                                } else {
                                    None
                                }
                            }
                            //("name", "=") => Some(Box::new($table::name.eq(value))),
                            //("name", "!=") => Some(Box::new($table::name.ne(value))),
                            //("name", "like") => Some(Box::new($table::name.like(value))),
                            _ => None, // ignore unsupported
                        };

                        if let Some(f) = filter {
                            filters.push(f);
                        }
                    }
                }

                filters
                    .into_iter()
                    .reduce(|acc, f| {
                        Box::new(acc.and(f))
                    })
                    .unwrap_or_else(|| Box::new(diesel::dsl::sql("1 = 1"))) // Silly, but allows reducing the amount of code needed
            }

            let (rows, items) = conn
                .run(move |c| {
                    let rows = $table
                        .filter(create_filter(&search_data))
                        .select(count_star()).first(c).expect("Issue");

                    if pagesize == -1 {
                        pagesize = rows;
                    }

                    let items = $table
                        .filter(create_filter(&search_data))
                        .limit(pagesize)
                        .offset(page * pagesize)
                        .select(<$t_output>::as_select())
                        .order_by($order_by)
                        .load(c)
                        .expect("Issue");

                    (rows, items)
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
        #[rocket::put($route, format = "json", data = "<data>")]
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
        #[rocket::delete($route)]
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
