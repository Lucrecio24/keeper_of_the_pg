pub mod players;
pub mod refresh;

use crate::discord_bot::*;
use std::collections::HashMap;
use serenity::{
    builder::{CreateCommand , CreateCommandOption},
    model::application::CommandOptionType, 
    all::CommandInteraction,
};


pub async fn run(
    ctx: &serenity::client::Context,
    command: &CommandInteraction,
    database: &sqlx::SqlitePool,
    handler_data: &HashMap<String, String>)
     -> CommandResponse {
        let mut mc_ip: &str = "192.168.1.169";
        if let Some(mc_query_ip) = handler_data.get("mc_query_ip") {
            mc_ip = mc_query_ip;
        } else {
            CommandResponse{
                result_string: "No hay una ip setupeada.".to_string(),
                ephemeral: true
                };
        }
        let mut mc_port: u16 = 25565_u16;
        if let Some(mc_query_ip) = handler_data.get("mc_query_ip") {
            match mc_query_ip.parse::<u16>() {
                Ok(port) => mc_port = port,
                Err(_) => {
                    CommandResponse{
                        result_string: "Error en pasar String a u16... Avisale al lucas".to_string(),
                        ephemeral: true
                        };
                }
            }
        } else {
            CommandResponse{
                result_string: "No hay un port setupeado.".to_string(),
                ephemeral: true
                };
        }


    // TODO: QUITAR MATCH EN COMANDO Y PEDIR A DISCORD QUE CHECKEE EL POR LOS PERMISOS ADECUADOS
    match command.data.options[0].name.as_str() {
        "players" => players::run(ctx , mc_ip , &mc_port).await,
        "refresh" => refresh::run(ctx , command , database).await,
        _ => {
            CommandResponse{
            result_string: "Subcomando no existente wtf.".to_string(),
            ephemeral: true
            }
        }
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("server")
    .description("test command please ignore")
    .add_option(CreateCommandOption::new(
        CommandOptionType::SubCommand,
            "players",
            "Get server player list",
        )
    .required(false))
}