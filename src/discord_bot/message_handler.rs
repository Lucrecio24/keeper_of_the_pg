extern crate serenity;

use tokio::time::{sleep , Duration};
use rand::Rng;
use serenity::{
    model::channel::{
        Message,
        ReactionType,
        MessageType,
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
        if msg.mentions.contains(&serenity::model::prelude::UserId::from(270781465896288256).to_user(&ctx).await.unwrap()) && msg.kind == MessageType::Regular {
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
            match msg.reply(&ctx, elected_photo).await {
                Err(error) => {println!("Error sending message {:?}" , error)}
                Ok(reply) => {
                    sleep(Duration::from_secs(5)).await;
                    _ = reply.delete(&ctx).await;
                }
            }
        }
        //Si contiene una mention al lanas, reaccionar con hakiri dance gif
        if msg.mentions.contains(&serenity::model::prelude::UserId::from(228684802520383489).to_user(&ctx).await.unwrap()) && msg.kind == MessageType::Regular {
            // Gif hakiri dance
            let elected_photo: &str = "https://tenor.com/gdXo1Jz9Bd4.gif";
            match msg.reply(&ctx, elected_photo).await {
                Err(error) => {println!("Error sending message {:?}" , error)}
                Ok(reply) => {
                    sleep(Duration::from_secs(5)).await;
                    _ = reply.delete(&ctx).await;
                }
            }
        }
        //Si contiene una mention al toty, reaccionar con la locura
        if msg.mentions.contains(&serenity::model::prelude::UserId::from(408088734722949132).to_user(&ctx).await.unwrap()) && msg.kind == MessageType::Regular {
            // Gif locura del payaso
            let elected_photo: &str = "https://tenor.com/nLfb5tspZ3j.gif";
            match msg.reply(&ctx, elected_photo).await {
                Err(error) => {println!("Error sending message {:?}" , error)}
                Ok(reply) => {
                    sleep(Duration::from_secs(5)).await;
                    _ = reply.delete(&ctx).await;
                }
            }
        }
        //Si contiene una mention al lucas
        if msg.mentions.contains(&serenity::model::prelude::UserId::from(228685282185052160).to_user(&ctx).await.unwrap()) && msg.kind == MessageType::Regular {
            {
            let rng = rand::thread_rng().gen_range(1..6);
            //println!("{}" , &rng);
            match rng {
                1..=4 => {}
                5 => {
                    match msg.reply(&ctx, "El admin").await {
                        Err(error) => {println!("Error sending message {:?}" , error)}
                        Ok(reply) => {
                            sleep(Duration::from_secs(5)).await;
                            _ = reply.delete(&ctx).await;
                        }
                    }
                }
                i32::MIN..=0_i32 | 6_i32..=i32::MAX => {println!("Number generated outside bounds")}
            }
            }
        }//Si contiene una mention al lucas
        if msg.mentions.contains(&serenity::model::prelude::UserId::from(762708012203769893).to_user(&ctx).await.unwrap()) && msg.kind == MessageType::Regular {
            match msg.reply(&ctx, "Soaquin Jolis").await {
                Err(error) => {println!("Error sending message {:?}" , error)}
                Ok(reply) => {
                    sleep(Duration::from_secs(5)).await;
                    _ = reply.delete(&ctx).await;
                }
           }
        }
    }
}