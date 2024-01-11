use crate::discord_bot::*;
use serenity::{
    builder::{CreateCommand , CreateCommandOption , CreateInteractionResponse , EditMessage , EditMember},
    model::application::CommandOptionType, 
    all::{CommandInteraction , Message , Mention}
};
/*
use serenity::{
    builder::CreateApplicationCommand,
    model::{
        mention::Mention,
        channel::{Message,Reaction},
        prelude::{
            command::CommandOptionType,
            application::interaction::InteractionResponseType,
            interaction::application_command::{
                ApplicationCommandInteraction,
                CommandDataOptionValue,
            }
        }
    },
    //prelude::EventHandler
}; 
*/

fn filter_function( reaction: &serenity::all::Reaction) -> bool{
    let partial_member = reaction.member.clone().unwrap();
    let roles_as_roleid = partial_member.roles;
    let mut is_op: bool = false;
    let mut is_correct_emoji: bool = false;

    //println!("{:?}" , reaction.emoji.as_data());
    
    //ADMIN RoleId = 715046083000139856   MOD RoleId = 715044649529770024
    for role in roles_as_roleid {
        if role.get() == 715046083000139856 || role.get() == 715044649529770024{
            is_op = true;
        }
    }

    if reaction.emoji.as_data().eq("%F0%9F%A5%B5") || reaction.emoji.as_data().eq("%F0%9F%A5%B6") {
        is_correct_emoji = true;
    }

    is_op && is_correct_emoji
}


pub async fn run(command: &CommandInteraction , ctx: &serenity::client::Context) -> Option<CommandResponse> {
    
    let admitted_channels = vec![959921482551668756 , 1087524950425997383];

    // COMMAND STILL IN DEVELOPMENT, THIS CHANNEL CHECK HAS TO GO.
    if !(admitted_channels.contains(&command.channel_id.get())){
        return Some(CommandResponse{
            result_string: format!("Canal equivocado. Prueba por {}" , serenity::model::id::ChannelId::from(1087524950425997383).to_channel(&ctx).await.unwrap()),
            ephemeral: true
        });
    }
    // TODO | Send a deffered update response, so later we can update it if the command was successful

    let mut new_nickname: String = "".to_string();
    let response_content: String;
    let mut is_nick_valid: bool = true;

    //ACTUAL COMMAND STARTS HERE

    //SAVE COMMAND DATA OPTIONS TO ITS OWN VARIABLE AND CHECK IF IT IS NONE
    match command.data.options.get(0) {
        //IF NONE, RETURN NICKNAME TO DEFAULT
        None => {
            new_nickname = "".to_string();
            response_content = "Sending request to remove nick".to_string();
        }
        //IF SOME, CHANGE NICKNAME TO INPUT STRING
        Some(value) => {
            let Some(option_value) = value.value.as_str() else {
                return Some(CommandResponse{
                    result_string: "Invalid name".to_string(),
                    ephemeral: true
                });
            };
            if new_nickname.clone().len() > 32 {
                new_nickname.truncate(32);
                response_content = "Name too long, but changing to truncated equivalent".to_string();
                is_nick_valid = false;
            } else {
                response_content = "Sending request to change nick".to_string();
                new_nickname = option_value.to_string();
            }
        }
    }
    let _ = command.create_response(ctx , 
        CreateInteractionResponse::Message(serenity::builder::CreateInteractionResponseMessage::new()
            .content(response_content)
            .ephemeral(true))).await;

    match is_nick_valid {
        true => {
            //If its valid then we do jack shit
        }
        false => {
            // return none if it isn't valid
            return None
        }
    }
    let message: Result<Message , serenity::prelude::SerenityError>;
    if new_nickname == *"" {
        let temp_message = command.channel_id.say(ctx , format!("{} quiere quitarse su apodo" , Mention::from(command.user.id))).await;
        message = temp_message;
    } else {
        let temp_message = command.channel_id.say(ctx , format!("{} quiere cambiar su apodo a {}" , Mention::from(command.user.id) , new_nickname)).await;
        message = temp_message;
    }
    let mut message_builder = EditMessage::new();
    
    let Ok(mut msg) = message else {
        println!("Failed to send message after callme command was executed. ERROR:\n{:?}" , message.err());
        return None;
    };
    //_reactions = msg.clone().await_reaction(ctx);

    let Some(reaction) = msg.await_reaction(ctx)
    .timeout(tokio::time::Duration::new(86400,0))
    .filter(|reaction| {
        filter_function(reaction)
    })
    .await else {
        // Case exists if await_reaction reaches timeout
        message_builder = message_builder.content(format!("{} no pudo cambiar su apodo a {}\nTiempo de espera sobrepasado..." , Mention::from(command.user.id) , new_nickname));
        let _msg = msg.edit(ctx, message_builder).await;
        return None;
    };
    let new_member_data = EditMember::new().nickname(&new_nickname);
    
    match reaction.emoji.as_data().as_str(){
        "%F0%9F%A5%B5" => {
            command.guild_id.unwrap().edit_member(&ctx , command.user.id , new_member_data).await.unwrap();
            if new_nickname.is_empty(){
                message_builder = message_builder.content(format!("{} se quitó su apodo\nCambio aprobado por {}" , Mention::from(command.user.id) , Mention::from(reaction.user_id.unwrap())));
                let _msg = msg.edit(ctx, message_builder).await;
                return None;
            } else {
                message_builder = message_builder.content(format!("{} cambió su apodo a {}\nCambio aprobado por {}" , Mention::from(command.user.id) , new_nickname , Mention::from(reaction.user_id.unwrap())));
                let _msg = msg.edit(ctx, message_builder).await;
                return None;
            }
        }
        "%F0%9F%A5%B6" => {
            if new_nickname.is_empty() {
                message_builder = message_builder.content(format!("{} no pudo quitarse su apodo\nCambio denegado por {}" , Mention::from(command.user.id) , Mention::from(reaction.user_id.unwrap())));
                let _msg = msg.edit(ctx, message_builder).await;
                return None;
            } else {
                message_builder = message_builder.content(format!("{} no pudo cambiar su apodo a {}\nCambio denegado por {}" , Mention::from(command.user.id) , new_nickname , Mention::from(reaction.user_id.unwrap())));
                let _msg = msg.edit(ctx, message_builder).await;
                return None;
            }
        }
        //Si discord devuelve que alguna reaccion que no esté registrada, mandamos error y avisamos por el chat
        _ => {
            println!("ERROR EN CALLME, EMOJI NO CORRESPONDIENTE");
            message_builder = message_builder.content("ERROR, diganle al Lucas y saquen screenshot pls".to_string());
            let _msg = msg.edit(ctx, message_builder).await;
            return None;
        }
    }

    // OLD CODE, COMMENT ASAP
    /*
    if let Some(reaction_action) = msg
        .await_reaction(ctx)
        .timeout(tokio::time::Duration::new(300,0))
        .filter(|reaction| {
            filter_function(reaction)
        })
        .await
        {

        if let serenity::collector::reaction_collector::ReactionAction::Added(reaction) = reaction_action.deref() {
            match reaction.emoji.as_data().as_str(){
                "🥵" => {
                    if new_nickname.is_empty(){
                        command.guild_id.unwrap().edit_member(&ctx , command.user.id , |m| m.nickname(&new_nickname)).await.unwrap();
                        let _msg = msg.edit(ctx, |new_msg|
                            new_msg.content(format!("{} se quitó su apodo\nCambio aprobado por {}" , Mention::from(command.user.id) , Mention::from(reaction.user_id.unwrap())))).await;
                        None
                    } else {
                        command.guild_id.unwrap().edit_member(&ctx , command.user.id , |m| m.nickname(&new_nickname)).await.unwrap();
                        let _msg = msg.edit(ctx, |new_msg|
                            new_msg.content(format!("{} cambió su apodo a {}\nCambio aprobado por {}" , Mention::from(command.user.id) , new_nickname , Mention::from(reaction.user_id.unwrap())))).await;
                        None
                    }
                }
                "🥶" => {
                    if new_nickname.is_empty() {
                        let _msg = msg.edit(ctx, |new_msg|
                            new_msg.content(format!("{} no pudo quitarse su apodo\nCambio denegado por {}" , Mention::from(command.user.id) , Mention::from(reaction.user_id.unwrap())))).await;
                        None
                    } else {
                        let _msg = msg.edit(ctx, |new_msg|
                            new_msg.content(format!("{} no pudo cambiar su apodo a {}\nCambio denegado por {}" , Mention::from(command.user.id) , new_nickname , Mention::from(reaction.user_id.unwrap())))).await;
                        None
                    }
                }
                //Si discord devuelve que alguna reaccion que no esté registrada, mandamos error y avisamos por el chat
                _ => {
                    println!("ERROR EN CALLME, EMOJI NO CORRESPONDIENTE");
                    let _msg = msg.edit(ctx, |new_msg|
                        new_msg.content("ERROR, diganle al Lucas".to_string())).await;
                    None
                }
            }
        } else {
            let _msg = msg.edit(ctx, |new_msg|
                new_msg.content("AAAAAAAAAAA WATAFAK".to_string())).await;
            println!("Reaction received: {:?}" , reaction_action);
            None
        }
        } else {
            // Case exists if await_reaction reaches timeout
            let _msg = msg.edit(ctx, |new_msg|
                new_msg.content(format!("{} no pudo cambiar su apodo a {}\nTiempo de espera sobrepasado..." , Mention::from(command.user.id) , new_nickname))).await;
            None
        } */
}



pub fn register() -> CreateCommand {
    CreateCommand::new("callme")
    .description("Change nickname of user")
    .add_option(CreateCommandOption::new(
        CommandOptionType::String,
            "nick",
            "New nickname",
        )
    .required(false))
}