#[macro_use] extern crate rocket;

mod routes;
mod utils;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/v1/", routes![routes::v1::get_status])
        .mount("/v1/platforms/", routes![routes::v1::platforms::list::get_list])
        .mount("/v1/platforms/steam/users/", routes![routes::v1::platforms::steam::users::get_users_list, routes::v1::platforms::steam::users::get_users_by_id, routes::v1::platforms::steam::users::get_users_count])
}
