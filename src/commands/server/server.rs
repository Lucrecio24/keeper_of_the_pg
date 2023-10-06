use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    ApplicationCommandInteraction,
//    CommandDataOptionValue,
};
//use serenity::model::mention::Mention;

use keeper_of_the_pg::*;
use crate::commands::server::*;

pub async fn run(
    ctx: &serenity::client::Context,
    command: &ApplicationCommandInteraction,
    database: &sqlx::SqlitePool,
    mc_ip: &String,
    mc_port: &u16)
     -> CommandResponse {

    // TODO: QUITAR MATCH EN COMANDO Y PEDIR A DISCORD QUE CHECKEE EL POR LOS PERMISOS ADECUADOS
    match command.data.options[0].name.as_str() {
        "players" => return players::run(ctx , mc_ip , mc_port).await,
        "refresh" => return refresh::run(ctx , command , database).await,
        _ => {
            return CommandResponse{
            result_string: format!("Subcomando no existente wtf."),
            ephemeral: true
            }
        }
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {

    command
        .name("server")
        .description("test command please ignore")
        .dm_permission(false)
        /*.create_option(|option| {
            option
                .name("list")
                .description("Get information available servers")
                .kind(CommandOptionType::SubCommand)
        })*/
        .create_option(|option| {
            option
                .name("players")
                .description("Get server player list")
                .kind(CommandOptionType::SubCommand)
        })
        /*.create_option(|option| {
            option
                .name("refresh")
                .description("Get information about a server")
                .kind(CommandOptionType::SubCommand)
        })*/
}