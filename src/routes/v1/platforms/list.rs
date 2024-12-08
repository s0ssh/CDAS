use rocket::serde::json::{json, Value};

#[get("/list")]
pub async fn get_list() -> Value {
    json!(["steam"])
}
