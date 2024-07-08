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

    if msg.author.id.get() == 1004145386887319692 {
    return;
    }
    
    // No idea what this is, referenced channel might not even exist
    if msg.channel_id == 1004982773121032242 {
        _ = msg.delete(&ctx).await;
        println!("Borré la wea de mensaje");
    }
    if msg.channel_id.get() == 1087524950425997383 {
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
    //Si contiene una mention al tommy, reaccionar impostor from among us
    if msg.mentions.contains(&serenity::model::prelude::UserId::from(597123683802415115).to_user(&ctx).await.unwrap()) && msg.kind == MessageType::Regular {
        // Gif amon gus?
        let elected_photo: &str = "https://tenor.com/bI7ht.gif";
        
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
        let elected_link: &str;
        {
            let mut rng = rand::thread_rng();
            let link_list = vec![
                "https://imgur.com/bM9S8QD",
                "https://tenor.com/gdXo1Jz9Bd4.gif"
                ];
            elected_link = link_list[rng.gen_range(0..2)];
        }
        match msg.reply(&ctx, elected_link).await {
            Err(error) => {println!("Error sending message {:?}" , error)}
            Ok(reply) => {
                sleep(Duration::from_secs(5)).await;
                _ = reply.delete(&ctx).await;
            }
        }
    }
    //Si contiene una mention al toty, reaccionar con la locura
    if msg.mentions.contains(&serenity::model::prelude::UserId::from(408088734722949132).to_user(&ctx).await.unwrap()) && msg.kind == MessageType::Regular {
        let elected_link: &str;
        {
            let mut rng = rand::thread_rng();
            let link_list = vec![
                "https://tenor.com/b1xD5.gif",
                "https://tenor.com/nLfb5tspZ3j.gif"
                ];
            elected_link = link_list[rng.gen_range(0..2)];
        }
        match msg.reply(&ctx, elected_link).await {
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
    }
    //Si contiene una mention al soaquin jolis
    if msg.mentions.contains(&serenity::model::prelude::UserId::from(762708012203769893).to_user(&ctx).await.unwrap()) && msg.kind == MessageType::Regular {
        let link: &str = "https://tenor.com/qME0wtX9xhG.gif";
        match msg.reply(&ctx, link).await {
            Err(error) => {println!("Error sending message {:?}" , error)}
            Ok(reply) => {
                sleep(Duration::from_secs(5)).await;
                _ = reply.delete(&ctx).await;
            }
       }
    }


    // la "anto" / BIG BLACK COCK https://imgur.com/5uHRsz8
    if msg.mentions.contains(&serenity::model::prelude::UserId::from(577666303326814238).to_user(&ctx).await.unwrap()) && msg.kind == MessageType::Regular {
        let big_black_cock: &str = "https://imgur.com/5uHRsz8";
        match msg.reply(&ctx, big_black_cock).await {
            Err(error) => {println!("Error sending message {:?}" , error)}
            Ok(reply) => {
                sleep(Duration::from_secs(5)).await;
                _ = reply.delete(&ctx).await;
            }
       }
    }

        // si contiene una mention al nacho jara / mewing snowman https://tenor.com/m0wi6ScLCqM.gif
    if msg.mentions.contains(&serenity::model::prelude::UserId::from(694606491864530964).to_user(&ctx).await.unwrap()) && msg.kind == MessageType::Regular {
        let big_black_cock: &str = "https://tenor.com/m0wi6ScLCqM.gif";
        match msg.reply(&ctx, big_black_cock).await {
            Err(error) => {println!("Error sending message {:?}" , error)}
            Ok(reply) => {
                sleep(Duration::from_secs(5)).await;
                _ = reply.delete(&ctx).await;
            }
       }
    }
        // si contiene una mention al croño / you need my dna gif https://tenor.com/fUxnw8FW0va.gif
    if msg.mentions.contains(&serenity::model::prelude::UserId::from(234812534522118145).to_user(&ctx).await.unwrap()) && msg.kind == MessageType::Regular {
        let big_black_cock: &str = "https://tenor.com/fUxnw8FW0va.gif";
        match msg.reply(&ctx, big_black_cock).await {
            Err(error) => {println!("Error sending message {:?}" , error)}
            Ok(reply) => {
                sleep(Duration::from_secs(5)).await;
                _ = reply.delete(&ctx).await;
            }
       }
    }
        // si contiene una mention al franco https://imgur.com/a/eBR7N7X
    if msg.mentions.contains(&serenity::model::prelude::UserId::from(553385188521148426).to_user(&ctx).await.unwrap()) && msg.kind == MessageType::Regular {
        let big_black_cock: &str = "https://imgur.com/Eb5OR8x";
        match msg.reply(&ctx, big_black_cock).await {
            Err(error) => {println!("Error sending message {:?}" , error)}
            Ok(reply) => {
                sleep(Duration::from_secs(5)).await;
                _ = reply.delete(&ctx).await;
            }
       }
    }
}