extern crate serenity;
use tokio::fs::File;
use std::io::Read;
use serenity::all::ResumedEvent;
pub async fn run(_bot: &crate::Handler, ctx: serenity::client::Context, _: ResumedEvent){
    
    let Some(current_ip) = public_ip::addr().await else {
        log::error!("Couldn't read current ip. Is file missing or unreadable?");
        return;
    };
    let mut tokio_file = match File::open("current_ip").await.expect("WTF").try_into_std() {
        Ok(file) => {file}
        Err(_error) => {
            return;
        }
    };
    let mut saved_ip: String = String::new();
    let _ = tokio_file.read_to_string(&mut saved_ip);

    let Ok(lucrecio_user) = serenity::model::id::UserId::from(228685282185052160).to_user(&ctx).await else {
        log::error!("Couldn't get Lucrecio's user");
        return;
    };
    if current_ip.to_string() == saved_ip{
        log::info!("Reconnected! IP untouched");
    } else {
        let Ok(_) = std::fs::write(std::path::Path::new("current_ip") , current_ip.to_string()) else {
            log::error!("Couldn't write current ip to file.");
            return;
        };
        if let Err(x) = lucrecio_user.direct_message(&ctx, serenity::builder::CreateMessage::new().content(format!("IP changed! {}" , current_ip.to_string()))).await {
            log::error!("Couldn't send new IP to Lucrecio.\n{x}");
        }
        log::info!("Reconnected! New IP is: {}", current_ip);
    }
}