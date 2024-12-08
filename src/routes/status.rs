use rocket::http::Status;

#[get("/status")]
pub async fn get_status() -> (Status, &'static str) {
    (Status::Ok, "ok")
}
