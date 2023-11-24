mod discord_bot;
mod axum_webserver;

use dotenvy::dotenv;
use tokio::task;
use serenity::async_trait;
use serenity::{
    model::{
        application::{
            command::Command,
            interaction::Interaction,
        },
        channel::Message,
        gateway::Ready,
        id::GuildId,
        guild::Member
    },
    prelude::*,
};
use std::env;

pub struct Handler {
    database: sqlx::SqlitePool,
    mc_ip: String,
    mc_port: u16
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        crate::discord_bot::message_handler::run(ctx,msg).await;
    }
    async fn guild_member_addition(&self , ctx: Context , new_member: Member){
        crate::discord_bot::new_member_handler::run(ctx , new_member).await;
    }
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        crate::discord_bot::interaction_handler::run(self , ctx , interaction).await;
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId(234453296545267714);

        let _guild_commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| discord_bot::commands::updatedb::register(command))
                .create_application_command(|command| discord_bot::commands::ping::register(command))
                .create_application_command(|command| discord_bot::commands::callme::register(command))
                .create_application_command(|command| discord_bot::commands::id::register(command))
                .create_application_command(|command| discord_bot::commands::insult::register(command))
                // Lanascoin command subcommands inside
                .create_application_command(|command| discord_bot::commands::lanascoin::lanascoin_handler::register(command))
                .create_application_command(|command| discord_bot::commands::server::server_handler::register(command))
            // Test command please ignore
            //.create_application_command(|command| commands::test::register(command))
        })
        .await;

        //        println!("I now have the following guild slash commands: {:#?}", commands);

        let _global_commands = Command::set_global_application_commands(&ctx.http, |commands| {
            commands
                //.create_application_command(|command| commands::wonderful_command::register(command))
                .create_application_command(|command| discord_bot::commands::ip::register(command))
        })
        .await;

        //        println!("I created the following global slash command: {:#?}", guild_command);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    // Configure the client with your Discord bot token in the environment.
    let token: String = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN must be set.");
    let _database_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

    // Setup database connection
    let database = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(10)
        .connect_with(
            sqlx::sqlite::SqliteConnectOptions::new()
                .filename("keepitpg.db")
                .create_if_missing(false))
        .await
        .expect("Couldn't connect to database");
    // Runs migrations
    sqlx::migrate!("./migrations")
        .run(&database)
        .await
        .expect("Couldn't run databse migrations");
    
    let mc_ip: String = String::from("keepitpg.xyz");
    let mc_port: u16 = 25569_u16;
    
    
    let bot = Handler { database , mc_ip , mc_port};

    // Build our client.
    let mut client = Client::builder(token, GatewayIntents::all())
        .event_handler(bot)
        .await
        .expect("Error creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
    
    let client_context = client.cache_and_http.http.clone();

    let bot_task = task::spawn(async move {
        if let Err(why) = client.start().await {
            println!("Client error: {:?}", why);
        }
    });
}
