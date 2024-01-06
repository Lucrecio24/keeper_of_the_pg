use crate::discord_bot::*;
use serenity::all::{CommandInteraction , Mention};


pub async fn run(
    ctx: &serenity::client::Context,
    command: &CommandInteraction,
    database: &sqlx::SqlitePool)
     -> CommandResponse {
    
    // Checking if actioner member is an admin
    match get_rank(ctx , *command.member.as_ref().unwrap().clone()).0 {
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

    // Saving command option as target user, or returning if invalid user (No idea if it is possible)
    let Some(target_user_id) = command.clone().data.options[0].value.as_user_id() else {
        return CommandResponse{
            result_string: "Miembro a buscar no ingresado".to_string(),
            ephemeral: true
        };
    };
    let Ok(member) = command.guild_id.unwrap().member(&ctx, &target_user_id).await else {
        return CommandResponse{
            result_string: "No se ha podido conectar con discord".to_string(),
            ephemeral: true
        };
    };
    let user_id_for_query = target_user_id.clone().to_string();
    let query_result = sqlx::query!(
        "SELECT * FROM members WHERE account_id = ?",
        user_id_for_query
    ).fetch_all(database).await;

    let Ok(query_result) = query_result else {
        return CommandResponse{
            result_string: "User not found in database...".to_string(),
            ephemeral: true
        };
    };
    let member_nick;
    if let Some(nick) = member.nick {
        member_nick = nick;
    } else {
        member_nick = "None".to_string();
    }
    return CommandResponse{
        result_string: format!(
            "Information about {}:\nCurrent nick: {}\nMax rank: {}\nLanasCoins: {}",
            Mention::from(target_user_id),
            member_nick,
            rank_to_string(serenity::model::id::RoleId::from(query_result.get(0).unwrap().rank_id.parse::<u64>().unwrap())),
            query_result.get(0).unwrap().lanas_coin
        ),
        ephemeral: true
    };
}




/*
    //OLD COMMAND
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
     */