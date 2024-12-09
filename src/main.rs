#[macro_use] extern crate rocket;

mod db;
mod routes;
mod utils;

use db::PgDb;
use lazy_static::lazy_static;
use tokio::sync::Mutex;

lazy_static! {
    static ref DB: Mutex<PgDb> =
        Mutex::new(PgDb::new().expect("Failed to init PgDb from env"));
}

#[launch]
async fn rocket() -> _ {
    dotenvy::dotenv().expect("Failed to init dotenvy from .env");
    DB.lock()
        .await
        .init_pool()
        .await
        .expect("Failed to init PgDb pool");

    DB.lock()
        .await
        .init_table_platform_steam_users()
        .await
        .expect("Failed to init PlatformSteamUsers table");


    rocket::build()
        .mount("/v1/", routes![routes::v1::get_status])
        .mount("/v1/platforms/", routes![routes::v1::platforms::list::get_list])
        .mount("/v1/platforms/steam/users/", routes![routes::v1::platforms::steam::users::get_users_list, routes::v1::platforms::steam::users::get_users_by_id, routes::v1::platforms::steam::users::get_users_count])
}
