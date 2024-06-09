use diesel::prelude::*;
use rocket::serde::json::Json;
use rocket_sync_db_pools::database;
use serde::Serialize;
use uuid::Uuid;

#[macro_use]
extern crate rocket;

mod schema;

#[database("itk_db")]
struct DbConnection(diesel::PgConnection);

#[derive(Identifiable, Queryable, Selectable, Serialize)]
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

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DbConnection::fairing())
        .mount("/", routes![my_route])
}
