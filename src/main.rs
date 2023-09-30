use anyhow::anyhow;
use prisma::{new_client_with_url, PrismaClient};
use regex::Regex;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use shuttle_secrets::SecretStore;
use std::sync::Arc;

#[allow(warnings, unused)]
mod prisma;

struct Handler;

struct DbConnection;

impl TypeMapKey for DbConnection {
    type Value = Arc<PrismaClient>;
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
        }

        kek_counter(&ctx, &msg).await;
        kek_fixer(&ctx, &msg).await;
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

async fn kek_counter(ctx: &Context, msg: &Message) {
    let db = {
        let data = ctx.data.read().await;
        data.get::<DbConnection>()
            .expect("Oops! Can't get db connection")
            .clone()
    };

    let has_kek = msg.content.to_lowercase().contains("kek");
    if has_kek {
        let kek_regex = Regex::new(r"(?mU).*:(?P<kek>.*[kK][eE][kK].*):.*").unwrap();
        let keks = kek_regex.captures_iter(&msg.content);
        let user = msg.author.name.as_str();

        for cap in keks {
            let kek = cap.name("kek").unwrap().as_str();
            println!("{}  {}", user, kek);
            let _ = db
                .kek_usage()
                .create(kek.to_string(), user.to_string(), vec![])
                .exec()
                .await;
        }
    }
}

async fn kek_fixer(ctx: &Context, msg: &Message) {
    let has_cursed_kek = msg.content.contains(":KEKWt:");
    let kek_suffer =
        "https://cdn.discordapp.com/emojis/1069468390231654561.webp?size=96&quality=lossless";
    if has_cursed_kek {
        let _ = msg.reply(ctx, kek_suffer).await;
    }
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
        token
    } else {
        return Err(anyhow!("'DISCORD_TOKEN' was not found").into());
    };

    // let db_url = secret_store
    //     .get("DATABASE_URL")
    //     .expect("URL for database missing! (env variable `DATABASE_URL`)");

    let db_url = if let Some(db_url) = secret_store.get("DATABASE_URL") {
        db_url
    } else {
        return Err(anyhow!("'DATABASE_URL' was not found").into());
    };

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    let db_client = new_client_with_url(&db_url)
        .await
        .expect("Database connection failed.");

    {
        let mut data = client.data.write().await;
        data.insert::<DbConnection>(Arc::new(db_client));
    }

    Ok(client.into())
}
