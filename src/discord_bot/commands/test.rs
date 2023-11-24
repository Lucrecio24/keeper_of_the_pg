use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    ApplicationCommandInteraction,
//    CommandDataOptionValue,
};
//use serenity::model::mention::Mention;

use keeper_of_the_pg::*;


pub async fn run(
    ctx: &serenity::client::Context,
    command: &ApplicationCommandInteraction,
    _database: &sqlx::SqlitePool)
     -> CommandResponse {

    // TODO: QUITAR MATCH EN COMANDO Y PEDIR A DISCORD QUE CHECKEE EL POR LOS PERMISOS ADECUADOS
    match get_rank(ctx , command.member.as_ref().unwrap().clone()).0 {
        Rank::Admin => {

        }

        _ => {
            return CommandResponse{
            result_string: format!("No tienes suficiente rango para usar este comando."),
            ephemeral: true
            }
        }
    }
    let nombre_subcomando: serenity::model::prelude::command::CommandOptionType = command.data.options.get(0).unwrap().kind;
    return CommandResponse{
        result_string: format!("Tipo de opcion: {:?}" , nombre_subcomando ),
        ephemeral: true
        }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {

    command
        .name("test")
        .description("test command please ignore")
        .dm_permission(false)
        .create_option(|option| {
            option
                .name("info")
                .description("Get info about user")
                .kind(CommandOptionType::SubCommand)
                .create_sub_option(|option| {
                    option
                        .name("user")
                        .description("user to get id from")
                        .kind(CommandOptionType::User)
                        .required(true)
                })
        })
        .create_option(|option| {
            option
                .name("inf")
                .description("gives info about user")
                .kind(CommandOptionType::SubCommand)
                .create_sub_option(|option| {
                    option
                        .name("member")
                        .description("member to get info from")
                        .kind(CommandOptionType::User)
                        .required(true)
                })
        })
}