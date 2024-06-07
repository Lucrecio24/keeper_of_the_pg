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
        "warframe" => erase_before_dm::run(ctx, command).await,
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

    CreateCommand::new("warframe")
    .description("Warframe API")
    .add_option(CreateCommandOption::new(
        CommandOptionType::SubCommand,
        "teshin",
        "Erases messages before a messageid in a private channel"))
}