use crate::discord_bot::*;

use serenity::{
        builder::{CreateCommand , CreateCommandOption},
        model::application::CommandOptionType, 
        all::CommandInteraction
};

pub async fn run(command: &CommandInteraction , ctx: &serenity::client::Context ) -> CommandResponse {
    let options = &command.data.options;

    let Some(option) = options.get(0) else {
        return CommandResponse{
            result_string: "Please provide a valid user".to_string(),
            ephemeral: true
        };  
    };
    let Some(user) = option.value.as_user_id() else {
        return CommandResponse{
            result_string: "Please provide a valid user".to_string(),
            ephemeral: true
        };
    };
    let Ok(user) = user.to_user(ctx).await else {
        return CommandResponse{
            result_string: "Couldn't connect to discord.\nPlease take screenshot and notify admin.".to_string(),
            ephemeral: true
        }; 
    };
    return CommandResponse{
        result_string: format!("{}'s id is {}", user.tag(), user.id),
        ephemeral: false
    };
}

pub fn register() -> CreateCommand {
    CreateCommand::new("id")
        .description("Get a user id")
        .add_option(CreateCommandOption::new(
            CommandOptionType::User,
            "id",
            "The user to lookup")
            .required(true))
}