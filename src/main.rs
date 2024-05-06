use crate::auth::service::AuthService;
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
