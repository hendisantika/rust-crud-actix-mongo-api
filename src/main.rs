use crate::auth::service::AuthService;
use crate::environment::Environment;
use crate::users::service::UserService;

mod environment;
mod errors;
mod auth;
mod users;
mod security;

mod test_controller;

pub struct AppState {
    auth_service: AuthService,
    user_service: UserService,
}

impl AppState {
    pub fn new(env: Environment) -> Self {
        AppState {
            auth_service: AuthService::new(env.clone()),
            user_service: UserService::new(env.clone()),
        }
    }
}
