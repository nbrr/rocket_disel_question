use diesel::prelude::*;
use rocket::serde::json::Json;
use serde::Serialize;
use uuid::Uuid;

#[macro_use]
extern crate rocket;

mod schema;

#[derive(Identifiable, Queryable, Selectable, Serialize)]
#[diesel(table_name=schema::my_elements)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct MyElement {
    id: Uuid,
    field: String,
}

#[get("/")]
fn my_route() -> Json<Vec<MyElement>> {
    let c = &mut establish_connection();

    let my_elements: Vec<MyElement> = schema::my_elements::table
        .select(MyElement::as_select())
        .load(c)
        .unwrap();

    Json(my_elements)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![my_route])
}

use diesel::pg::PgConnection;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
