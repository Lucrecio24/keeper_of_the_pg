extern crate serenity;

use serenity::{
    model::channel::{
        Message,
        ReactionType
    },
    utils::parse_emoji
};

pub async fn run(ctx: serenity::client::Context, msg: Message){

    if (msg.channel_id == 1004982773121032242) && (msg.author.id != 1004145386887319692) {
        _ = msg.delete(&ctx).await;
        println!("Borré la wea de mensaje");
    }
    if msg.content.to_lowercase().contains("toty") && (msg.author.id != 1004145386887319692) {
        match msg.channel_id.say(&ctx, "El toty se la come").await {
            Err(error) => println!("Error sending message {:?}", error),
            _ => {}
        }
    }
    if msg.content.to_lowercase().contains("toti") & (msg.author.id != 1004145386887319692) {
        match msg.channel_id.say(&ctx.http, "Es con y").await {
            Err(error) => println!("Error sending message {:?}", error),
            _ => {}
        }
    }
    if msg.content.to_lowercase().contains("dbd"){
        match msg.react(ctx, ReactionType::from(parse_emoji("<:tired:1036702905530597447>").unwrap())).await {
            Err(error) => println!("Error sending message {:?}" , error),
            _ => {}
        }
    }
}