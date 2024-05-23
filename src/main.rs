mod discord_bot;
mod axum_webserver;

use dotenvy::dotenv;
use serenity::all::ResumedEvent;
use serenity::utils::ArgumentConvert;
use tokio::task;
use serenity::async_trait;
use serenity::{
        model::{
            application::Interaction,
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
    data: std::collections::HashMap<String , String>,
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
    async fn guild_audit_log_entry_create(&self , ctx: Context , entry: serenity::model::guild::audit_log::AuditLogEntry , guild_id: GuildId){
        crate::discord_bot::updated_member_handler::run(ctx , entry , guild_id).await;
    }
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        crate::discord_bot::interaction_handler::run(self , ctx , interaction).await;
    }
    async fn resume(&self, _ctx: Context, _: ResumedEvent) {
        let current_ip = public_ip::addr().await.unwrap();
        println!("Reconnected! current ip is: {}", current_ip);
    }
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        crate::discord_bot::commands::mass_registering(&ctx).await;

        let guild_id = GuildId::from(234453296545267714);    
        match serenity::model::channel::Message::convert(&ctx, Some(guild_id) , Some(serenity::model::id::ChannelId::from(991080906200596511)), "1196194295674310758").await {
            Ok(mut rolebutton_message) => {
                _ = rolebutton_message.edit(&ctx, crate::discord_bot::button_handler::role_message_builder().await).await;
            }
            Err(error) => {
                println!("{:?}" , error)
            }
        }
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
    

    let client_context = client.http.clone();
    let bot_task = task::spawn(async move {
        if let Err(why) = client.start().await {
            println!("Client error: {:?}", why);
        }
    });
    let axum_task = task::spawn(async move {
        if let Err(why) = crate::axum_webserver::start_webserver(client_context).await{
            println!("Couldn't start webserver: {:?}" , why);
        }
    });
    tokio::try_join!(bot_task , axum_task).unwrap();
}