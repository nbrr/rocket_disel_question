use diesel::prelude::*;
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rocket::serde::json::Json;
use rocket_db_pools::{Connection, Database};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[macro_use]
extern crate rocket;

mod schema;

#[derive(Database)]
#[database("itk_db")]
struct DbConnection(Pool<AsyncPgConnection>);

#[derive(Identifiable, Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name=schema::my_elements)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct MyElement {
    id: Uuid,
    field: String,
}

#[get("/")]
async fn my_route(mut conn: Connection<DbConnection>) -> Json<Vec<MyElement>> {
    let my_elements: Vec<MyElement> = schema::my_elements::table
        .select(MyElement::as_select())
        .load(&mut **conn)
        .await
        .unwrap();

    Json(my_elements)
}

#[post("/", format = "application/json", data = "<my_new_element>")]
async fn my_post_route(
    mut conn: Connection<DbConnection>,
    my_new_element: Json<MyElement>,
) -> Json<MyElement> {
    let my_element = diesel::insert_into(schema::my_elements::table)
        .values(&my_new_element.0)
        .get_result(&mut **conn)
        .await
        .unwrap();

    Json(my_element)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DbConnection::init())
        .mount("/", routes![my_route, my_post_route])
}
