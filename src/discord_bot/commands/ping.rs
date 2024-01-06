use crate::discord_bot::*;
use serenity::builder::CreateCommand;
use serenity::model::application::CommandDataOption;

pub fn run(_options: &[CommandDataOption]) -> CommandResponse {
    CommandResponse{
        result_string: "Pong!".to_string(),
        ephemeral: true
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("ping")
        .description("A ping command")
}