use actix_web::{Error, HttpMessage};
use actix_web::dev::ServiceRequest;
use actix_web::web::Data;
use bcrypt::hash;
use chrono::{Duration, Utc};

use crate::AppState;
use crate::auth::models::Claims;
use crate::errors::GenericError;
use crate::users::models::User;

//
// jwt methods
//
fn get_secret() -> Vec<u8> {
    std::env::var("JWT_SECRET").unwrap().into_bytes()
}