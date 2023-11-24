use crate::discord_bot::*;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

pub fn run(_options: &[CommandDataOption]) -> CommandResponse {
    CommandResponse{
        result_string: "Pong!".to_string(),
        ephemeral: true
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("ping").description("A ping command")
}