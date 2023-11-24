use crate::discord_bot::*;
use serenity::model::prelude::interaction::application_command::{
    ApplicationCommandInteraction,
    CommandDataOptionValue,
};
use serenity::model::prelude::Mention;

pub async fn run(
    ctx: &serenity::client::Context,
    command: &ApplicationCommandInteraction,
    database: &sqlx::SqlitePool)
     -> CommandResponse {
    
    // Checking if actioner member is an admin
    match get_rank(ctx , command.member.as_ref().unwrap().clone()).0 {
        Rank::Admin => {
            //If admin we continue with the program
        }
        _ => {
            return CommandResponse{
            result_string: "No tienes suficiente rango para usar este comando.".to_string(),
            ephemeral: true
            }
        }
    }
        // Checking if command data was inputted, and returning if none
    if command.data.options.get(0).is_none() {
        return CommandResponse{
            result_string: "Miembro a buscar no ingresado".to_string(),
            ephemeral: true
        }
    }
    // Saving command option under option variable
    let option = &command.data.options[0].options[0].resolved
    .as_ref()
    .unwrap();
    
    if let CommandDataOptionValue::User(user, member) = option {
        //Saving user id to make query. Then await and save query.

        let member_id = user.id.as_u64().to_string();
        let query_result = sqlx::query!(
            "SELECT * FROM members WHERE account_id = ?",
            member_id
        )
        .fetch_all(database)
        .await;
        
        // If the query gave something, print the result back, with the corresponding data
        if let Ok(result) = query_result {
            // Getting member nickname
            let member_nick;
            if let Some(nick) = member.clone().unwrap().nick {
                member_nick = nick;
            } else {
                member_nick = "None".to_string();
            }
            CommandResponse{
                result_string: format!(
                    "Information about {}:\nCurrent nick: {}\nMax rank: {}\nLanasCoins: {}",
                    Mention::from(user.id),
                    member_nick,
                    rank_to_string(serenity::model::id::RoleId(result.get(0).unwrap().rank_id.parse::<u64>().unwrap())),
                    result.get(0).unwrap().lanas_coin
                ),
                ephemeral: true
            }
        } else {
            //If query didn't yield, inform user
            CommandResponse{
                result_string: "User not found in database...".to_string(),
                ephemeral: true
            }
        }
    } else {
                //If query didn't yield, inform user
            CommandResponse{
                result_string: "User not found in database...".to_string(),
                ephemeral: true
            }
    }
}
