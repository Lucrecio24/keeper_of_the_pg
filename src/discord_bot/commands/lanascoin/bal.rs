use crate::discord_bot::*;
use serenity::all::CommandInteraction;

// Didn't rewrite completely, just made compilable

pub async fn run(
    _ctx: &serenity::client::Context ,
    command: &CommandInteraction,
    database: &sqlx::SqlitePool
    ) -> CommandResponse {
    
    let user_id = command.user.id.to_string();
    let query_result = sqlx::query!(
        "SELECT lanas_coin FROM members WHERE account_id = ?",
        user_id
    ).fetch_one(database).await;

    if let Ok(result) = query_result {
        CommandResponse{
            result_string: format!("Tienes {} LanasCoin" , result.lanas_coin),
            ephemeral: true
        }    
    } else {
        CommandResponse{
            result_string: "No estas en la base de datos...".to_string(),
            ephemeral: true
        }    
    }
}