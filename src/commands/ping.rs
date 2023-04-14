use keeper_of_the_pg::*;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::{CommandDataOption};

pub fn run(_options: &[CommandDataOption]) -> CommandResponse {
    return CommandResponse{
        result_string: format!("Pong!"),
        ephemeral: true
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("ping").description("A ping command")
}