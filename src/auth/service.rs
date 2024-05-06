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

    pub fn login(&self, request: AuthRequest) -> Result<AuthResponse, GenericError> {
        // Find user by email
        let filter = doc! {"email": &request.email };
        let existing: User = match self.collection.find_one(filter, None).unwrap() {
            Some(obj) => from_document(obj).unwrap(),
            None => return Err(GenericError { message: "Not found" }),
        };

        // Validate passwords
        match verify_password(&request.password, &existing.password) {
            true => (),
            false => return Err(GenericError { message: "Invalid credentials" }),
        };

        self.generate_tokens_and_update(existing)
    }

    /**
    Used by jwt_middleware to check if token has not been revoked
     */
    pub fn validate(&self, token: &str) -> Result<User, GenericError> {
        // todo!("Get session from faster store such as redis");
        let filter = doc! { "tokens.access_token": &token };
        let user: User = match self.collection.find_one(filter, None).unwrap() {
            Some(obj) => from_document(obj).unwrap(),
            None => return Err(GenericError { message: "Not found" }),
        };
        Ok(user)
    }

    /**
    Generates new tokens if the given refresh_token is valid
     */
    pub fn refresh(&self, token: &str) -> Result<AuthResponse, GenericError> {
        // todo!("Get session from faster store such as redis");
        let filter = doc! { "tokens.refresh_token": &token };
        let user: User = match self.collection.find_one(filter, None).unwrap() {
            Some(obj) => from_document(obj).unwrap(),
            None => return Err(GenericError { message: "Not found" }),
        };
        self.generate_tokens_and_update(user)
    }
}