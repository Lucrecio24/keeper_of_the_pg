use keeper_of_the_pg::*;
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
    }
};
use std::ops::Deref;
use std::sync::Arc;
/*async fn reaction_by_op( ctx: &Context , reaction: Reaction) -> bool{
    let member = Member::convert(ctx , reaction.guild_id , _ , &reaction.user_id.unwrap().0.to_string().as_str()).await;
    let max_rank: Rank;
    match member{
        Ok(x) => max_rank = get_rank(ctx, x).0,
        Err(error) => {}
    }
    match max_rank {
        Rank::Admin | Rank::Mod => true,
        _ => false
    }
}*/

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
        return Some(CommandResponse{
            result_string: format!("Comando no implementado aún, paciencia porfa"),
            ephemeral: true
        })
    } else {
        // TODO | Send a deffered update response, so later we can update it if the command was successful
        if let Err(why) = command
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| 
                    message
                    .content("Sending message and waiting for reactions".to_string())
                    .ephemeral(true))
        })
        .await
        {
            println!("Couldn't respond to slash command:\n{}", why);
        }
        let mut new_nickname: String = "".to_string();
        //ACTUAL COMMAND STARTS HERE
        
        //SAVE COMMAND DATA OPTIONS TO ITS OWN VARIABLE AND CHECK IF IT IS NONE
        match command.data.options.get(0) {
            None => {}
            Some(value) => {
                if let CommandDataOptionValue::String(new_nickname_string) = value.resolved.as_ref().unwrap(){
                    new_nickname = new_nickname_string.to_string();
                }
            }
        }
        let message: Result<Message , serenity::prelude::SerenityError>;
        let _reactions;
        if new_nickname == "".to_string() {
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
                        if let serenity::collector::reaction_collector::ReactionAction::Added(reaction) = reaction_action.deref().clone(){
                            match reaction.emoji.as_data().as_str(){
                                "🥵" => {
                                    if new_nickname == ""{
                                        command.guild_id.unwrap().edit_member(&ctx , command.user.id , |m| m.nickname(&new_nickname)).await.unwrap();
                                        let _msg = msg.edit(ctx, |new_msg|
                                            new_msg.content(format!("{} se quitó su apodo\nCambio aprobado por {}" , Mention::from(command.user.id) , Mention::from(reaction.user_id.unwrap())))).await;
                                        return None
                                    } else {
                                        command.guild_id.unwrap().edit_member(&ctx , command.user.id , |m| m.nickname(&new_nickname)).await.unwrap();
                                        let _msg = msg.edit(ctx, |new_msg|
                                            new_msg.content(format!("{} cambió su apodo a {}\nCambio aprobado por {}" , Mention::from(command.user.id) , new_nickname , Mention::from(reaction.user_id.unwrap())))).await;
                                        return None
                                    }
                                }
                                "🥶" => {
                                    if new_nickname == "" {
                                        let _msg = msg.edit(ctx, |new_msg|
                                            new_msg.content(format!("{} no pudo quitarse su apodo\nCambio denegado por {}" , Mention::from(command.user.id) , Mention::from(reaction.user_id.unwrap())))).await;
                                        return None
                                    } else {
                                        let _msg = msg.edit(ctx, |new_msg|
                                            new_msg.content(format!("{} no pudo cambiar su apodo a {}\nCambio denegado por {}" , Mention::from(command.user.id) , new_nickname , Mention::from(reaction.user_id.unwrap())))).await;
                                        return None
                                    }
                                }
                                //Si discord devuelve que alguna reaccion que no esté registrada, mandamos error y avisamos por el chat
                                _ => {
                                    println!("ERROR EN CALLME, EMOJI NO CORRESPONDIENTE");
                                    let _msg = msg.edit(ctx, |new_msg|
                                        new_msg.content(format!("ERROR, diganle al Lucas"))).await;
                                    return None
                                }
                            }
                        } else {
                            let _msg = msg.edit(ctx, |new_msg|
                                new_msg.content(format!("AAAAAAAAAAA WATAFAK"))).await;
                            println!("Reaction received: {:?}" , reaction_action);
                            None
                        }
                    } else {
                        // Case exists if await_reaction reaches timeout
                        let _msg = msg.edit(ctx, |new_msg|
                            new_msg.content(format!("{} no pudo cambiar su apodo a {}\nTiempo de espera sobrepasado..." , Mention::from(command.user.id) , new_nickname))).await;
                        return None
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