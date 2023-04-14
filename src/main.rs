use keeper_of_the_pg::*;
mod commands;

use dotenvy::dotenv;
use std::env;
use serenity::async_trait;
use serenity::{
        model::{
            application::{
                command::Command,
                interaction::{Interaction , InteractionResponseType}
            },
            channel::Message,
            gateway::Ready,
            id::GuildId
        },
        prelude::*
    };

struct Handler{
    database: sqlx::SqlitePool
}

#[async_trait]
impl EventHandler for Handler {

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.to_lowercase().contains("toty") & (msg.author.id != 1004145386887319692) {
            match msg.channel_id.say(&ctx.http, "El toty se la come").await {
                Err(error) => println!("Error sending message {:?}", error),
                _ => {}
            }
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {

            let keeper_of_the_pg_channel = serenity::model::id::ChannelId(1087524950425997383).to_channel(&ctx).await.unwrap();
            if ((command.channel_id == 1087524950425997383) || (command.channel_id == 959921482551668756)) || (command.guild_id.is_none()) {

                // Receiving of non-reaction-dependant commands processed here
                let content: CommandResponse = match command.data.name.as_str() {
                    "ping" => commands::ping::run(&command.data.options),
                    "id" => commands::id::run(&command.data.options),
                    "ip" => commands::ip::run(&command , &ctx).await,
                    "updatedb" => commands::updatedb::run(&ctx , &command , &self.database).await,
                    "insult" => commands::insult::run(&command , &ctx , &self.database).await,
                    "callme" => commands::callme::run(&command , &ctx).await,
                    // Lanas coin command subcommands inside
                    "lanascoin" => commands::lanascoin::lanascoin::run(&ctx , &command , &self.database).await,
                    // Test command please ignore
                    //"test" => commands::test::run(&ctx , &command , &self.database).await,

                    _ => {
                        println!("ERROR 100 INTERACCIÓN NO EXISTENTE");
                        CommandResponse{
                            result_string: format!("Comando no existente..."),
                            ephemeral: true
                        }
                    }
                };
                if let Err(why) = command
                    .create_interaction_response(&ctx.http, |response| {
                        response
                            .kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|message| 
                                message
                                .content(content.result_string)
                                .ephemeral(content.ephemeral))
                    })
                    .await
                    {
                        println!("Couldn't respond to slash command:\n{}", why);
                    }
            } else {

                if let Err(why) = command
                .create_interaction_response( &ctx.http , |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| 
                            message
                                .content(format!("Canal equivocado. Prueba por {}" , keeper_of_the_pg_channel))
                                .ephemeral(true)
                            )
                })
                .await
                {
                    println!("Cannot respond to slash command: {}", why);
                }
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
                .create_application_command(|command| commands::lanascoin::lanascoin::register(command))
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
                .create_if_missing(false)
        )
        .await
        .expect("Couldn't connect to database");
    // Runs migrations
    sqlx::migrate!("./migrations").run(&database).await.expect("Couldn't run databse migrations");
    let bot = Handler{
        database
    };

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