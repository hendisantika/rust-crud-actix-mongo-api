use bson::Document;
use mongodb::sync::Collection;

use crate::environment::Environment;
use crate::security::{get_jwt_for_user, verify_password};

#[derive(Clone)]
pub struct AuthService {
    collection: Collection<Document>,
}

impl AuthService {
    pub fn new(env: Environment) -> AuthService {
        let collection: Collection<Document> = env.db().collection("users");
        AuthService { collection }
    }
}