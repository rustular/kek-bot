use regex::Regex;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

mod env;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
        }

        kek_counter(&msg);
        kek_fixer(&ctx,&msg).await;
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn kek_counter(msg: &Message) {
    let has_kek = msg.content.to_lowercase().contains("kek");
    if has_kek {
        let kek_regex = Regex::new(r"(?mU).*:(?P<kek>.*[kK][eE][kK].*):.*").unwrap();
        let keks = kek_regex.captures_iter(&msg.content);
        let user = msg.author.name.as_str();

        for cap in keks {
            let kek = cap.name("kek").unwrap().as_str();
            println!("{}  {}", user, kek);
        }
    }
}

async fn kek_fixer( ctx: &Context,msg: &Message) {
    let has_cursed_kek = msg.content.contains(":KEKWt:");
    let kek_suffer =
        "https://cdn.discordapp.com/emojis/1069468390231654561.webp?size=96&quality=lossless";
    if has_cursed_kek {
        msg.reply(ctx, kek_suffer).await;
    }
}

#[tokio::main]
async fn main() {
    let token = env::get_env().unwrap().discord_token;
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
