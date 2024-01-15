use crate::discord_bot::*;
use serenity::all::CommandInteraction;

pub async fn run(
    ctx: &serenity::client::Context,
    command: &CommandInteraction,
    _database: &sqlx::SqlitePool)
     -> CommandResponse {
    
    // Checking if actioner member is an admin or mod
    match get_rank(ctx , *command.member.as_ref().unwrap().clone()).0 {
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
