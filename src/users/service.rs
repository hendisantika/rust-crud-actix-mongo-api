use bson::Document;
use mongodb::sync::Collection;

use crate::errors::GenericError;
use crate::security::get_hashed_password;

#[derive(Clone)]
pub struct UserService {
    collection: Collection<Document>,
}

impl UserService {
    pub fn new(env: Environment) -> UserService {
        let collection: Collection<Document> = env.db().collection("users");
        UserService { collection }
    }
}