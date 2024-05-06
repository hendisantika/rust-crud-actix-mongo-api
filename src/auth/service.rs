use bson::Document;
use mongodb::sync::Collection;

use crate::security::{get_jwt_for_user, verify_password};

#[derive(Clone)]
pub struct AuthService {
    collection: Collection<Document>,
}