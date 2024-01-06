pub mod players;
pub mod refresh;

use crate::discord_bot::*;
use serenity::{
    builder::{CreateCommand , CreateCommandOption},
    model::application::CommandOptionType, 
    all::CommandInteraction,
};

//use crate::discord_bot::commands::server::*;

pub async fn run(
    ctx: &serenity::client::Context,
    command: &CommandInteraction,
    database: &sqlx::SqlitePool,
    mc_ip: &str,
    mc_port: &u16)
     -> CommandResponse {

    // TODO: QUITAR MATCH EN COMANDO Y PEDIR A DISCORD QUE CHECKEE EL POR LOS PERMISOS ADECUADOS
    match command.data.options[0].name.as_str() {
        "players" => players::run(ctx , mc_ip , mc_port).await,
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