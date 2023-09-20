mod env;

use serenity::async_trait;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::model::channel::Message;

use serenity::model::id::EmojiId;
use serenity::model::misc::EmojiIdentifier;

use serenity::prelude::*;
#[group]
#[commands(ping, wrong_kek)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let env = env::get_env().unwrap();

    let framework = StandardFramework::new()
        .configure(|c| {
            c.prefixes.push("".to_owned());
            c
        })
        .group(&GENERAL_GROUP);

    let token = env.discord_token;
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;
    Ok(())
}

#[command]
#[aliases(":KEKWt:")]
async fn wrong_kek(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(
        ctx,
        "https://cdn.discordapp.com/emojis/1069468390231654561.webp?size=96&quality=lossless",
    )
    .await?;
    Ok(())
}
