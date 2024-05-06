use actix_web::{get, HttpResponse, Responder};
// use actix_web_grants::proc_macro::has_any_permission;
// use actix_web_grants::proc_macro::has_roles;
// use actix_web_grants::proc_macro::protect;
use actix_web_grants::protect;
use serde_json::json;

use crate::users::models::User;

#[get("/api/public")]
pub async fn public(current_user: User) -> impl Responder {
    HttpResponse::Ok().body(json!({ "username": &current_user.username, "endpoint_security": "PUBLIC" }).to_string())
}

#[get("/api/protected/user")]
#[protect("ROLE_ADMIN","ROLE_USER")]
pub async fn protected_user(current_user: User) -> impl Responder {
    HttpResponse::Ok().body(json!({ "username": &current_user.username, "endpoint_security": "ANY ROLE" }).to_string())
}

#[get("/api/protected/admin")]
#[protect("ROLE_ADMIN")]
pub async fn protected_admin(current_user: User) -> impl Responder {
    HttpResponse::Ok().body(json!({ "username": &current_user.username, "endpoint_security": "ROLE_ADMIN" }).to_string())
}