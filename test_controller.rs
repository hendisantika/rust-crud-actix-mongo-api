use actix_web::{get, HttpResponse, Responder};
use actix_web_grants::proc_macro::has_any_permission;
use serde_json::json;

use crate::users::models::User;

#[get("/api/public")]
pub async fn public(current_user: User) -> impl Responder {
    HttpResponse::Ok().body(json!({ "username": &current_user.username, "endpoint_security": "PUBLIC" }).to_string())
}