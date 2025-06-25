#[macro_use]
extern crate rocket;

mod godwords;

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![godwords::get_godwords])
}
