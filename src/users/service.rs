use bson::Document;
use mongodb::sync::Collection;

use crate::errors::GenericError;
use crate::security::get_hashed_password;

#[derive(Clone)]
pub struct UserService {
    collection: Collection<Document>,
}