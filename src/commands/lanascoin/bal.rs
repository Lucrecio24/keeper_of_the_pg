use keeper_of_the_pg::*;
use serenity::model::prelude::interaction::application_command::{
    ApplicationCommandInteraction,
};


pub async fn run(
    _ctx: &serenity::client::Context ,
    command: &ApplicationCommandInteraction,
    database: &sqlx::SqlitePool
    ) -> CommandResponse {
    
    let user_id = command.user.id.as_u64().to_string();
    let query_result = sqlx::query!(
        "SELECT lanas_coin FROM members WHERE account_id = ?",
        user_id
    )
    .fetch_one(database)
    .await;

    if let Ok(result) = query_result {
        return CommandResponse{
            result_string: format!("Tienes {} LanasCoin" , result.lanas_coin),
            ephemeral: true
        }    
    } else {
        return CommandResponse{
            result_string: format!("No estas en la base de datos..."),
            ephemeral: true
        }    
    }
}