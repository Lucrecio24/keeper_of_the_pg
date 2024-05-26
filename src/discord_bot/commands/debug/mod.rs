pub mod erase_before_dm;

use crate::discord_bot::*;
use serenity::{
    builder::{CreateCommand , CreateCommandOption},
    model::application::CommandOptionType, 
    all::CommandInteraction,
};

pub async fn run(
    ctx: &serenity::client::Context,
    command: &CommandInteraction,
    _database: &sqlx::SqlitePool)
     -> CommandResponse {

    match command.data.options[0].name.as_str() {
        "erase_before_dm" => add::run(ctx, command).await,
        _ => {
            log::warn!("Subcommand '{}' doesn't exist" , command.data.options[0].name.as_str());
            CommandResponse{
            result_string: "Nonexistant subcommand".to_string(),
            ephemeral: true
            }
        }
    }
}

pub fn register() -> CreateCommand {

    CreateCommand::new("debug")
    .description("Command for debugging purposes")
    .add_option(CreateCommandOption::new(
        CommandOptionType::SubCommand,
        "erase_before_dm",
        "Erases messages before a messageid in a private channel"))
        .add_sub_option(CreateCommandOption::new(
            CommandOptionType::User,
            "user",
            "target user")
            .required(true))
        .add_sub_option(CreateCommandOption::new(
            CommandOptionType::User,
            "message_id",
            "id of the reference message")
            .required(true))
    /*.add_option(CreateCommandOption::new(
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
            .required(true)))*/
}