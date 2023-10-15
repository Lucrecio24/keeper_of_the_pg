extern crate serenity;

use rand::Rng;
use serenity::{
    model::channel::{
        Message,
        ReactionType
    },
    utils::parse_emoji
};

pub async fn run(ctx: serenity::client::Context, msg: Message){
    // author.id 1004145386887319692 is the bot itself.
    // Below, we check if the bot himself didn't say the words that trigger the response.
    // Endless loops could be triggered elseway

    if msg.author.id != 1004145386887319692 {

        // No idea what this is, referenced channel might not even exist
        if msg.channel_id == 1004982773121032242 {
            _ = msg.delete(&ctx).await;
            println!("Borré la wea de mensaje");
        }
        if msg.content.to_lowercase().contains("toty") {
            if let Err(error) = msg.channel_id.say(&ctx, "El toty se la come").await {
                println!("Error sending message {:?}", error);
            }
        }
        if msg.content.to_lowercase().contains("toti") {
            if let Err(error) = msg.channel_id.say(&ctx, "Es con y").await {
                println!("Error sending message {:?}", error);
            }
        }
        if msg.content.to_lowercase().contains("primo god") {
            if let Err(error) = msg.channel_id.say(&ctx, "Primo god").await {
                println!("Error sending message {:?}", error);
            }
        }        
        if msg.content.to_lowercase().contains("franco") {
            if let Err(error) = msg.channel_id.say(&ctx, "FrancoGPT").await {
                println!("Error sending message {:?}", error);
            }
        }
        if msg.content.to_lowercase().contains("chechi") {
            if let Err(error) = msg.channel_id.say(&ctx, "Chechi barre tu meao del piso").await {
                println!("Error sending message {:?}", error);
            }
        }
        if msg.content.to_lowercase().contains("dbd") {
            if let Err(error) = msg.react(&ctx, ReactionType::from(parse_emoji("<:tired:1036702905530597447>").unwrap())).await {
                println!("Error sending message {:?}" , error)
            }
        }
        if msg.mentions.contains(&serenity::model::prelude::UserId::from(270781465896288256).to_user(&ctx).await.unwrap()) {
        // Fotos huevo con tocino
            let elected_photo: &str;
            {
                let mut rng = rand::thread_rng();
                let beok_photo_list = vec![
                    "https://imgur.com/QaAgTKA",
                    "https://imgur.com/yHzrmsp",
                    "https://imgur.com/3rjJec4",
                    "https://imgur.com/7qtPQWf",
                    "https://imgur.com/Mvrl8bU",
                    "https://imgur.com/mANUwCc"];
                elected_photo = beok_photo_list[rng.gen_range(0..6)];
            }
            if let Err(error) = msg.reply(&ctx, elected_photo).await {
                    println!("Error sending message {:?}" , error)
            }
        }
        if msg.mentions.contains(&serenity::model::prelude::UserId::from(228684802520383489).to_user(&ctx).await.unwrap()) {
            // Fotos huevo con tocino
            let elected_photo: &str = "https://tenor.com/gdXo1Jz9Bd4.gif";
            if let Err(error) = msg.reply(ctx, elected_photo).await {
                    println!("Error sending message {:?}" , error)
            }
        }
    }
}