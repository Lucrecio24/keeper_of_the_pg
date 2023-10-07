use keeper_of_the_pg::*;

//use serenity::builder::CreateApplicationCommand;
//use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    ApplicationCommandInteraction,
    //CommandDataOptionValue,
};
//use serenity::model::prelude::Mention;

pub async fn run(
    ctx: &serenity::client::Context,
    command: &ApplicationCommandInteraction,
    _database: &sqlx::SqlitePool)
     -> CommandResponse {
    
    // Checking if actioner member is an admin or mod
    match get_rank(ctx , command.member.as_ref().unwrap().clone()).0 {
        Rank::Admin | Rank::Mod => {
            //If admin or mod we continue with the program
        }
        _ => {
            return CommandResponse{
            result_string: "No tienes suficiente rango para usar este comando.".to_string(),
            ephemeral: true
            }
        }
    }


    
    CommandResponse {
        result_string: "Command unfinished".to_string(),
        ephemeral: true
    }
}
