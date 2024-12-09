use crate::DB;

use rocket::http::Status;
use rocket::serde::{
    json::{json, Json, Value},
    Deserialize, Serialize,
};

#[get("/list?<page>&<per_page>")]
pub async fn get_users_list(page: Option<usize>, per_page: Option<usize>) -> (Status, Value) {
    let page = page.unwrap_or(0);
    let per_page = per_page.unwrap_or(1000);

    todo!()
}

#[get("/<steam_id>")]
pub async fn get_users_by_id(steam_id: u64) -> (Status, Value) {

    todo!()
}

#[get("/count")]
pub async fn get_users_count() -> (Status, Value) {
    match DB.lock().await.query_table_platform_steam_users_count().await {
        Ok(result) => (Status::Ok, json!({"count": result})),
        Err(_) => (Status::InternalServerError, json!({})),
    }
}

// TODO! add insert entry endpoint
