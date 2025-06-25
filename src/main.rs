#[macro_use]
extern crate rocket;

mod routes;

#[launch]
async fn rocket() -> _ {
    dotenvy::dotenv().expect("Failed to init dotenvy from .env");

    rocket::build()
        .mount("/", routes![routes::get_status, routes::wisdom::get_wisdom])
}
