use crate::discord_bot::*;
use serenity::all::CommandInteraction;
use serenity::model::id::MessageId;

pub async fn run(
    ctx: &serenity::client::Context,
    command: &CommandInteraction)
    -> CommandResponse {
    
    // This command gets every message before a certain message id in a private chat and deletes them.
    // Get confirm actioner permission -> get target user -> get messages -> delete messages


    // Checking if actioner member is Lucrecio (228685282185052160)
    if command.user.id.get() != 228685282185052160 {
        return CommandResponse{
            result_string: "No eres el Lucrecio.".to_string(),
            ephemeral: true
            }
    }
    let command_options: Vec<serenity::model::application::CommandDataOption>;
    if let serenity::model::application::CommandDataOptionValue::SubCommand(temp) = command.data.options[0].value.clone() {
        command_options = temp;
    } else {
        log::warn!("SubCommandDataOption couldn't be extracted");
        return CommandResponse{
            result_string: "Error interno, avisa a Lucrecio".to_string(),
            ephemeral: true
        };
    }
    // Saving command option as target user, or returning if invalid user (No idea if it is possible)
    let Some(target_user) = command_options[0].value.as_user_id() else {
        return CommandResponse{
            result_string: "Miembro no ingresado".to_string(),
            ephemeral: true
        };
    };
    // Saving command option as target user, or returning if invalid user (No idea if it is possible)
    let Some(target_message_id) = command_options[1].value.as_str() else {
        return CommandResponse{
            result_string: "Id de mensaje no ingresado".to_string(),
            ephemeral: true
        };
    };
    let Ok(target_message_id) = target_message_id.parse::<u64>() else {
        log::warn!("Couldn't parse target_message_id into u64");
        return CommandResponse{
            result_string: "Error interno, avisa a Lucrecio".to_string(),
            ephemeral: true
        };
    };
    // Get dm_channel for target_user
    let Ok(target_channel) = target_user.create_dm_channel(&ctx).await else {
        log::warn!("Couldn't get dm_channel for target_user.");
        return CommandResponse{
            result_string: "Error interno, avisale al Lucrecio".to_string(),
            ephemeral: true
        };
    };
    // Get vec messages containing everything before target_message_id
    let Ok(messages_vec) = target_channel.messages(&ctx , serenity::builder::GetMessages::new().before(MessageId::new(target_message_id)).limit(100)).await else {
        log::warn!("Couldn't get vec of messages before target_id");
        return CommandResponse{
            result_string: "Error interno, avisale al Lucrecio".to_string(),
            ephemeral: true
        };
    };
    let Ok(_) = target_channel.delete_messages(&ctx, messages_vec).await else {
        log::warn!("Couldn't delete messages before target_id");
        return CommandResponse{
            result_string: "Error interno, avisale al Lucrecio".to_string(),
            ephemeral: true
        };
    };
    return CommandResponse{
        result_string: "Mensajes borrados exitosamente".to_string(),
        ephemeral: true
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