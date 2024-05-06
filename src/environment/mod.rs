use mongodb::sync::Client;

#[derive(Clone)]
pub struct Environment {
    db_pool: Client,
    config: Config,
}

#[derive(Clone, Debug)]
pub struct Config {
    db_name: String,
    pub host: String,
    // ... other properties to be used throughout the app
}

impl Environment {}