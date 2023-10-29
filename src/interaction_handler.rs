use serenity::{
    model::
        application::interaction::{Interaction, InteractionResponseType},
    prelude::*,
};
use keeper_of_the_pg::CommandResponse;

pub async fn run(bot: &crate::Handler , ctx: Context, interaction: Interaction) {
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
                    "callme" => crate::commands::callme::run(&command, &ctx).await,
                    // Receiving of non-reaction-dependant commands (nrd commands) processed here
                    "ping" => Some(crate::commands::ping::run(&command.data.options)),
                    "id" => Some(crate::commands::id::run(&command.data.options)),
                    "ip" => Some(crate::commands::ip::run(&command, &ctx).await),
                    "updatedb" => Some(crate::commands::updatedb::run(&ctx, &command, &bot.database).await),
                    "insult" => Some(crate::commands::insult::run(&command, &ctx, &bot.database).await),
                    // Lanas coin command subcommands inside
                    "lanascoin" => Some(crate::commands::lanascoin::lanascoin_handler::run(&ctx, &command, &bot.database).await),
                    "server" => Some(crate::commands::server::server_handler::run(&ctx, &command, &bot.database, &bot.mc_ip , &bot.mc_port).await),
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