pub mod add;
pub mod info;
pub mod set;
pub mod bal;

use crate::discord_bot::*;
use serenity::{
    builder::{CreateCommand , CreateCommandOption},
    model::application::CommandOptionType, 
    all::CommandInteraction,
};

//use crate::discord_bot::commands::lanascoin::*;

pub async fn run(
    ctx: &serenity::client::Context,
    command: &CommandInteraction,
    database: &sqlx::SqlitePool)
     -> CommandResponse {

    // TODO: QUITAR MATCH EN COMANDO Y PEDIR A DISCORD QUE CHECKEE EL POR LOS PERMISOS ADECUADOS
    match command.data.options[0].name.as_str() {
        "info" => info::run(ctx , command , database).await,
        "set" => set::run(ctx , command , database).await,
        "bal" => bal::run(ctx , command , database).await,
        "add" => add::run(ctx, command, database).await,
        _ => {
            CommandResponse{
            result_string: "Subcomando no existente wtf.".to_string(),
            ephemeral: true
            }
        }
    }
}

pub fn register() -> CreateCommand {

    CreateCommand::new("lanascoin")
    .description("test command please ignore")
    .add_option(CreateCommandOption::new(
        CommandOptionType::SubCommand,
        "bal",
        "Get own balance"))
    .add_option(CreateCommandOption::new(
        CommandOptionType::SubCommand,
        "info",
        "Get information about user")
        .add_sub_option(CreateCommandOption::new(
            CommandOptionType::User,
            "user",
            "target user")
            .required(true)))
    .add_option(CreateCommandOption::new(
        CommandOptionType::SubCommand,
        "set",
        "Set lanascoins to certain amount")
        .add_sub_option(CreateCommandOption::new(
            CommandOptionType::User,
            "user",
            "target user")
            .required(true)))
    .add_option(CreateCommandOption::new(
        CommandOptionType::SubCommand,
        "add",
        "Set lanascoins to certain amount")
        .add_sub_option(CreateCommandOption::new(
            CommandOptionType::User,
            "user",
            "target user")
            .required(true))
        .add_sub_option(CreateCommandOption::new(
            CommandOptionType::Integer,
            "amount",
            "target amount of LanasCoin")
            .required(true)))
}