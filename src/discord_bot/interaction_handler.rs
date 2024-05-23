use serenity::{
    builder::{CreateInteractionResponse , CreateInteractionResponseMessage},
    all::Interaction,
    prelude::*,
};
use crate::discord_bot::CommandResponse;
use crate::discord_bot::commands as commands;

pub async fn run(bot: &crate::Handler , ctx: Context, interaction: Interaction) {
    match interaction {
        Interaction::Ping(_) => {println!("Ping interaction")}
        Interaction::Command(command) => {
            let content: Option<CommandResponse>;
            let valid_channels: Vec<u64> = vec![1087524950425997383 , 959921482551668756 , 1004982773121032242];
            let Ok(keeper_of_the_pg_channel) = serenity::model::id::ChannelId::from(1087524950425997383).to_channel(&ctx).await else {
                todo!()
            };
            let mut command_response = CreateInteractionResponseMessage::new();


            if !(valid_channels.contains(&command.channel_id.get())) {    
                command_response = command_response.content(format!("Canal equivocado. Prueba por {}", keeper_of_the_pg_channel)).ephemeral(true);
                let _ = command.create_response(&ctx , CreateInteractionResponse::Message(command_response.clone())).await;
            }

            content = match command.data.name.as_str() {
                // Receiving of reaction-dependant commands (rd commands) processed here
                "callme" => commands::callme::run(&command, &ctx).await,
                // Receiving of non-reaction-dependant commands (nrd commands) processed here
                "ping" => Some(commands::ping::run(&command.data.options)),
                "id" => Some(commands::id::run(&command , &ctx).await),
                "ip" => Some(commands::ip::run(&command, &ctx).await),
                "updatedb" => Some(commands::updatedb::run(&ctx, &command, &bot.database).await),
                "insult" => Some(commands::insult::run(&command, &ctx, &bot.database).await),
                // Lanas coin command subcommands inside
                "lanascoin" => Some(commands::lanascoin::run(&ctx, &command, &bot.database).await),
                "server" => Some(commands::server::run(&ctx, &command, &bot.database, &bot.data).await),
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

            let Some(content) = content else {
                return ();
            };

            command_response = command_response.content(content.result_string).ephemeral(content.ephemeral);
            let _ = command.create_response(&ctx , CreateInteractionResponse::Message(command_response.clone())).await;

        }
        Interaction::Component(component_data) => {
            let custom_id = component_data.data.custom_id.clone();
            let mut custom_id = custom_id.split("_");
            let Some(firstword) = custom_id.nth(0) else {
                return;
            };
            let Some(secondword) = custom_id.nth(0) else {
                return;
            };
            
            match firstword {
                "rolebutton" => {
                    crate::discord_bot::button_handler::rolebutton(ctx , component_data , secondword).await;
                }
                _ => {
                    let response_message = CreateInteractionResponseMessage::new().content("Botón no configurado").ephemeral(true);
                    let response = CreateInteractionResponse::Message(response_message);
                    _ = component_data.create_response(ctx, response).await;
                }
            }
        }

        _ => {
            //Do nothing if interaction isn't one of the previously mentioned
        }
    }
}
