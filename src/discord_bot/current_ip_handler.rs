extern crate serenity;

use serenity::all::ResumedEvent;

pub async fn run(bot: &crate::Handler, ctx: serenity::client::Context, _: ResumedEvent){

    let mut data = bot.data.write().await;

        let Some(current_ip) = public_ip::addr().await else {
            return;
        };
        let Some(saved_ip) = data.get("saved_ip") else {
            data.insert(String::from("saved_ip"), current_ip.to_string());
            println!("New IP!: {}", current_ip);
            let Ok(lucrecio_user) = serenity::model::id::UserId::from(228685282185052160).to_user(&ctx).await else {
                println!("Couldn't get Lucrecio's user");
                return;
            };
            if let Err(x) = lucrecio_user.direct_message(&ctx, serenity::builder::CreateMessage::new().content(format!("IP changed! {}" , current_ip.to_string()))).await{
                println!("Couldn't send new IP to Lucrecio.\nERORR: {x}");
            }
            return;
        };
        if &current_ip.to_string() == saved_ip {
            return;
        }
        println!("Reconnected! current ip is: {}", current_ip);
}