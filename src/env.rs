use dotenv::{dotenv, Error};
use std::env;

pub struct Env {
    pub discord_token: String,
    pub database_url: String
}

pub fn get_env() -> Result<Env, Error> {
    dotenv().ok();

    Ok(Env {
        discord_token: env::var("DISCORD_TOKEN").expect("Expected a Discord token in the environment"),
        database_url: env::var("DATABASE_URL").expect("Expected a DATABASE_URL in the environment")
    })
}
