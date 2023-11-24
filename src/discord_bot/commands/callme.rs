use crate::discord_bot::*;
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
use std::ops::Deref;
use std::sync::Arc;

fn filter_function( reaction: &Arc<Reaction>) -> bool{
    let partial_member = reaction.member.clone().unwrap();
    let roles_as_roleid = partial_member.roles;
    let mut is_op: bool = false;
    let mut is_correct_emoji: bool = false;

    for role in roles_as_roleid {
        if role.0 == 715046083000139856 || role.0 == 715044649529770024{
            is_op = true;
        }
    }

    if reaction.emoji.as_data().eq("🥵") || reaction.emoji.as_data().eq("🥶") {
        is_correct_emoji = true;
    }

    is_op && is_correct_emoji
}


pub async fn run(command: &ApplicationCommandInteraction , ctx: &serenity::client::Context) -> Option<CommandResponse> {
    

    // COMMAND STILL IN DEVELOPMENT, THIS CHANNEL CHECK HAS TO GO.
    if command.channel_id != 959921482551668756 {
        Some(CommandResponse{
            result_string: format!("Canal equivocado. Prueba por {}" , serenity::model::id::ChannelId(1087524950425997383).to_channel(&ctx).await.unwrap()),
            ephemeral: true
        })
    } else {
        // TODO | Send a deffered update response, so later we can update it if the command was successful

        let mut new_nickname: String = "".to_string();
        let mut response_content: String = "WTF?".to_string();
        let mut is_nick_valid: bool = true;

        //ACTUAL COMMAND STARTS HERE

        //SAVE COMMAND DATA OPTIONS TO ITS OWN VARIABLE AND CHECK IF IT IS NONE
        match command.data.options.get(0) {
            None => {
                response_content = "Sending request to remove nick".to_string();
            }
            Some(value) => {
                if let CommandDataOptionValue::String(new_nickname_string) = value.resolved.as_ref().unwrap(){
                    //IF NEW NICK ISN'T 32 OR LESS, RETURN WITH ERROR
                    if new_nickname_string.len() > 32 {
                        response_content = "Nick can't be longer than 32 characters".to_string();
                        is_nick_valid = false;
                    } else {
                        response_content = "Sending request to change nick".to_string();
                        new_nickname = new_nickname_string.to_string();
                    }
                }
            }
        }
        let _ = command
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| 
                    message
                    .content(response_content)
                    .ephemeral(true))
        })
        .await;
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
        let _reactions;
        if new_nickname == *"" {
            let temp_message = command.channel_id.say(ctx , format!("{} quiere quitarse su apodo" , Mention::from(command.user.id))).await;
            message = temp_message;
        } else {
            let temp_message = command.channel_id.say(ctx , format!("{} quiere cambiar su apodo a {}" , Mention::from(command.user.id) , new_nickname)).await;
            message = temp_message;
        }
        match message {
            Err(error) => {
                println!("Failed to send message after callme command was executed. ERROR:\n{:?}" , error);
                None
            }
            Ok(mut msg) => {
                _reactions = msg.clone().await_reaction(ctx);
                if let Some(reaction_action) = msg
                    .await_reaction(ctx)
                    .timeout(tokio::time::Duration::new(30,0))
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
                    }
            }
        }
    }
}



pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("callme").description("Change nickname of user").create_option(|option| {
        option
            .name("nick")
            .description("New nickname")
            .kind(CommandOptionType::String)
            .required(false)
    })
}