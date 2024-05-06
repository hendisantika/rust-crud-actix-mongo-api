use std::fmt;

use actix_web::{FromRequest, HttpMessage, HttpRequest};
use actix_web::dev::Payload;
use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: Option<ObjectId>,
    pub email: String,
    pub username: String,
    pub password: String,
    pub roles: Vec<Role>,
    pub tokens: Option<Tokens>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

// Retrieves user object from session previously set by the jwt middleware to be available for injection on controllers
impl FromRequest for User {}