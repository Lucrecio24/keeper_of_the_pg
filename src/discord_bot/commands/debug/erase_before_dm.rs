use crate::discord_bot::*;
use serenity::all::{CommandInteraction , Mention};


pub async fn run(
    ctx: &serenity::client::Context,
    command: &CommandInteraction)
    -> CommandResponse {
    
    // This command gets every message before a certain message id in a private chat and deletes them.
    // Get confirm actioner permission -> get target user -> get messages -> delete messages


    // Checking if actioner member is Lucrecio (228685282185052160)
    if command.member.id.get() != 228685282185052160 {
        return CommandResponse{
            result_string: "No eres el Lucrecio.".to_string(),
            ephemeral: true
            }
    }
    // Saving command option as target user, or returning if invalid user (No idea if it is possible)
    let Some(target_user_id) = command.data.options[0].value.clone().as_user_id() else {
        return CommandResponse{
            result_string: "Miembro a buscar no ingresado".to_string(),
            ephemeral: true
        };
    };
    /*
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
    */
}