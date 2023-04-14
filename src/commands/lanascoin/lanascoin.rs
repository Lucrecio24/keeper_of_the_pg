use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    ApplicationCommandInteraction,
//    CommandDataOptionValue,
};
//use serenity::model::mention::Mention;

use keeper_of_the_pg::*;
use crate::commands::lanascoin::*;

pub async fn run(
    ctx: &serenity::client::Context,
    command: &ApplicationCommandInteraction,
    database: &sqlx::SqlitePool)
     -> CommandResponse {

    // TODO: QUITAR MATCH EN COMANDO Y PEDIR A DISCORD QUE CHECKEE EL POR LOS PERMISOS ADECUADOS
    match command.data.options[0].name.as_str() {
        "info" => return info::run(ctx , command , database).await,
        "set" => return set::run(ctx , command , database).await,
        "bal" => return bal::run(ctx , command , database).await,
        "add" => return add::run(ctx, command, database).await,
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
        .name("lanascoin")
        .description("test command please ignore")
        .dm_permission(false)
        .create_option(|option| {
            option
                .name("bal")
                .description("Get own balance")
                .kind(CommandOptionType::SubCommand)
        })
        .create_option(|option| {
            option
                .name("info")
                .description("Get information about user")
                .kind(CommandOptionType::SubCommand)
                .create_sub_option(|option| {
                    option
                        .name("user")
                        .description("target user")
                        .kind(CommandOptionType::User)
                        .required(true)
                })
        })
        .create_option(|option| {
            option
                .name("set")
                .description("Set lanascoins to certain amount")
                .kind(CommandOptionType::SubCommand)
                .create_sub_option(|option| {
                    option
                        .name("user")
                        .description("target user")
                        .kind(CommandOptionType::User)
                        .required(true)
                })
                .create_sub_option(|option| {
                    option
                        .name("amount")
                        .description("target amount of LanasCoin")
                        .kind(CommandOptionType::Integer)
                        .required(true)
                })
        })
        .create_option(|option| {
            option
                .name("add")
                .description("add lanascoins to a user")
                .kind(CommandOptionType::SubCommand)
                .create_sub_option(|option| {
                    option
                        .name("user")
                        .description("target user")
                        .kind(CommandOptionType::User)
                        .required(true)
                })
                .create_sub_option(|option| {
                    option
                        .name("amount")
                        .description("amount to add")
                        .kind(CommandOptionType::Integer)
                        .required(true)
                })
        })
}