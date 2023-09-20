use dotenv::{dotenv, Error};
use std::env;

pub struct Env {
    pub discord_token: String,
}

pub fn get_env() -> Result<Env, Error> {
    dotenv().ok();

    let discord_token =
        env::var("DISCORD_TOKEN").expect("Expected a Discord token in the environment");

    Ok(Env {
        discord_token: discord_token,
    })
}
