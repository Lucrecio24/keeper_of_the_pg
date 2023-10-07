use keeper_of_the_pg::*;
mod commands;
mod message_handler;

use dotenvy::dotenv;
use serenity::async_trait;
use serenity::{
    model::{
        application::{
            command::Command,
            interaction::{Interaction, InteractionResponseType},
        },
        channel::Message,
        gateway::Ready,
        id::GuildId
    },
    prelude::*,
};
use std::env;

struct Handler {
    database: sqlx::SqlitePool,
    mc_ip: String,
    mc_port: u16
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        crate::message_handler::run(ctx,msg).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction {
            Interaction::Ping(_) => {println!("Ping interaction")}
            Interaction::ApplicationCommand(command) => {
                let content: Option<CommandResponse>;
                let valid_channels: Vec<u64> = [1087524950425997383 , 959921482551668756 , 1004982773121032242].to_vec();

                let keeper_of_the_pg_channel = serenity::model::id::ChannelId(1087524950425997383)
                    .to_channel(&ctx)
                    .await
                    .unwrap();
                if (valid_channels.contains(&command.channel_id.0)) || (command.guild_id.is_none()) {
                    content = match command.data.name.as_str() {
                        // Receiving of reaction-dependant commands (rd commands) processed here
                        "callme" => commands::callme::run(&command, &ctx).await,
                        // Receiving of non-reaction-dependant commands (nrd commands) processed here
                        "ping" => Some(commands::ping::run(&command.data.options)),
                        "id" => Some(commands::id::run(&command.data.options)),
                        "ip" => Some(commands::ip::run(&command, &ctx).await),
                        "updatedb" => Some(commands::updatedb::run(&ctx, &command, &self.database).await),
                        "insult" => Some(commands::insult::run(&command, &ctx, &self.database).await),
                        // Lanas coin command subcommands inside
                        "lanascoin" => Some(commands::lanascoin::lanascoin_handler::run(&ctx, &command, &self.database).await),
                        "server" => Some(commands::server::server_handler::run(&ctx, &command, &self.database, &self.mc_ip , &self.mc_port).await),
                        // Test command please ignore
                        //"test" => commands::test::run(&ctx , &command , &self.database).await,
                        _ => {
                            println!("ERROR 100 INTERACCIÓN NO EXISTENTE");
                            Some(CommandResponse {
                                result_string: "Comando no existente...".to_string(),
                                ephemeral: true,
                            })
                        }
                    };
                    if let Some(content) = content {
                        if let Err(why) = command
                            .create_interaction_response(&ctx.http, |response| {
                                response
                                    .kind(InteractionResponseType::ChannelMessageWithSource)
                                    .interaction_response_data(|message| {
                                        message
                                            .content(content.result_string)
                                            .ephemeral(content.ephemeral)
                                    })
                            })
                            .await
                        {
                            println!("Couldn't respond to slash command:\n{}", why);
                        }
                    }
                } else if let Err(why) = command
                    .create_interaction_response(&ctx.http, |response| {
                        response
                            .kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|message| {
                                message
                                    .content(format!(
                                        "Canal equivocado. Prueba por {}",
                                        keeper_of_the_pg_channel
                                    ))
                                    .ephemeral(true)
                            })
                    })
                    .await
                {
                    println!("Cannot respond to slash command: {}", why);
                }
            }
            _ => {
                //Do nothing if interaction isn't one of the previously mentioned
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId(234453296545267714);

        let _guild_commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| commands::updatedb::register(command))
                .create_application_command(|command| commands::ping::register(command))
                .create_application_command(|command| commands::callme::register(command))
                .create_application_command(|command| commands::id::register(command))
                .create_application_command(|command| commands::insult::register(command))
                // Lanascoin command subcommands inside
                .create_application_command(|command| commands::lanascoin::lanascoin_handler::register(command))
                .create_application_command(|command| commands::server::server_handler::register(command))
            // Test command please ignore
            //.create_application_command(|command| commands::test::register(command))
        })
        .await;

        //        println!("I now have the following guild slash commands: {:#?}", commands);

        let _global_commands = Command::set_global_application_commands(&ctx.http, |commands| {
            commands
                //.create_application_command(|command| commands::wonderful_command::register(command))
                .create_application_command(|command| commands::ip::register(command))
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
}
