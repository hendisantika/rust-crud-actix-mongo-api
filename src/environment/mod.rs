use mongodb::sync::Client;

#[derive(Clone)]
pub struct Environment {
    db_pool: Client,
    config: Config,
}