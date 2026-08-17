use poise::serenity_prelude as serenity;
use dotenvy::dotenv;
use std::env;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

mod commands;

pub struct Data {}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("TOKEN").expect("failed to read token");
    let guildid = serenity::GuildId::new(env::var("GUILDID").expect("failed to read token").parse().expect("failed to convert from String"));
    let intents = serenity::GatewayIntents::empty();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands::register_commands(),
            ..Default::default()
        })
        .setup(move | ctx, _ready, framework | {
            Box::pin(async move {
                poise::builtins::register_in_guild(ctx, &framework.options().commands, guildid.into()).await.expect("failed to register commands");
                println!("╔═══════════════════════════════════════════════════════");
                println!("║ Embers v0.1.0 - Online ");
                println!("╚═════════════════════════════════════════");

                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap()

}
