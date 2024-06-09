use rocket::serde::json::Json;
use rocket_db_pools::{diesel::prelude::*, diesel::PgPool, Connection, Database};
use serde::Serialize;
use uuid::Uuid;

#[macro_use]
extern crate rocket;

mod schema;

#[derive(Database)]
#[database("itk_db")]
struct DbConnection(PgPool);

#[derive(Identifiable, Queryable, Selectable, Serialize)]
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

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DbConnection::init())
        .mount("/", routes![my_route])
}
