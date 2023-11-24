use crate::discord_bot::*;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption,
    CommandDataOptionValue,
};


pub fn run(options: &[CommandDataOption]) -> CommandResponse {
    let option = options.get(0)
        .expect("Expected user option")
        .resolved
        .as_ref()
        .expect("Expected user object");
    if let CommandDataOptionValue::User(user, _member) = option {

        CommandResponse{
            result_string: format!("{}'s id is {}", user.tag(), user.id),
            ephemeral: false
        }
    } else {
        CommandResponse{
            result_string: "Please provide a valid user".to_string(),
            ephemeral: true
        }
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("id")
        .description("Get a user id").create_option(|option| {
            option
                .name("id")
                .description("The user to lookup")
                .kind(CommandOptionType::User)
                .required(true)
    })
}