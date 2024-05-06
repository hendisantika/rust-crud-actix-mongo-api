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

    pub fn create(&self, request: CreateUser) -> Result<InsertOneResult, GenericError> {
        let filter = bson::doc! {"username": &request.username };
        // let filter = bson::doc! {"$or": [{"email": &request.email }, {"username": &request.username }]};
        match self.collection.find_one(filter, None).unwrap() {
            Some(_) => return Err(GenericError { message: "User already exists" }),
            None => (),
        }

        let user = User {
            id: None,
            email: request.email,
            username: request.username,
            password: get_hashed_password(&request.password),
            roles: request.roles,
            tokens: None,
            created_at: Some(Utc::now()),
            updated_at: None,
        };

        let mut doc: Document = to_document(&user).unwrap();
        doc.remove("_id"); // Remove None field that would be saved instead of auto-generated

        let result: Result<InsertOneResult, Error> = self.collection.insert_one(doc, None);
        Ok(result.unwrap())
    }
}