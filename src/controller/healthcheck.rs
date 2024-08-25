

use actix_web::{get, web, Responder};
use chrono::Utc;
use serde_json::json;

// get current time in YYYY-MM-DDTHH:MM:SSZ format
#[get("/")]
pub async fn healthcheck() -> impl Responder {
    web::Json(json!({
        "status": "ok",
        "current_time": Utc::now().to_rfc2822()
    }))
}