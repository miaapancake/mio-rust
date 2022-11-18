#![allow(unused)]
mod commands;
mod config;
mod models;

use std::env;

use deadpool_postgres::Pool;
use serenity::async_trait;
use serenity::model::application::command::Command;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::model::prelude::Message;
use serenity::prelude::*;

use dotenv::dotenv;

use tokio_postgres::{ Error, };

use config::db::PgConfig;


pub struct Handler {
    pub db_pool: Pool
}

impl Handler {

    pub fn new(db_pool: Pool) -> Self {
        Self {
            db_pool: db_pool
        }
    }

}

#[async_trait]
impl EventHandler for Handler {

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {

        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {}", command.data.name);
            
            let content = match command.data.name.as_str() {
                "rankroles" => commands::rankroles::run(&command, &self).await,
                _ => "not implemented :(".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn message(&self, ctx: Context, new_message: Message) {
        print!("{}", new_message.author.tag());
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);


        let guild_command = Command::create_global_application_command(&ctx.http, |command| {
            commands::rankroles::register(command)
        })
        .await;

        println!("I created the following global slash command: {:#?}", guild_command);
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    
    if let Ok(_) = dotenv() {
        // do nothing on error, because it doesn't matter
        // if dotenv isn't loaded, we load from env anyway
    };

    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let db_pool = PgConfig::new().make_db_pool().await;

    // Build our client.
    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler::new(db_pool))
        .await
        .expect("Error creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }

    Ok(())
}