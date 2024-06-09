use diesel::prelude::*;
use rocket::serde::json::Json;
use rocket_sync_db_pools::database;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[macro_use]
extern crate rocket;

mod schema;

#[database("itk_db")]
struct DbConnection(diesel::PgConnection);

#[derive(Identifiable, Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name=schema::my_elements)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct MyElement {
    id: Uuid,
    field: String,
}

#[get("/")]
async fn my_route(conn: DbConnection) -> Json<Vec<MyElement>> {
    conn.run(|c| {
        let my_elements: Vec<MyElement> = schema::my_elements::table
            .select(MyElement::as_select())
            .load(c)
            .unwrap();

        Json(my_elements)
    })
    .await
}

#[post("/", format = "application/json", data = "<my_new_element>")]
async fn my_post_route(conn: DbConnection, my_new_element: Json<MyElement>) -> Json<MyElement> {
    conn.run(move |c| {
        let my_element = diesel::insert_into(schema::my_elements::table)
            .values(&my_new_element.0)
            .get_result(c)
            .unwrap();

        Json(my_element)
    })
    .await
}
#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DbConnection::fairing())
        .mount("/", routes![my_route, my_post_route])
}
