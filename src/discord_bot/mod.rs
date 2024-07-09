pub mod message_handler;
pub mod interaction_handler;
pub mod new_member_handler;
pub mod updated_member_handler;
pub mod commands;
pub mod button_handler;



extern crate dotenvy;
use serenity::all::RoleId;
use tokio::fs::File;
use std::io::Read;

pub struct CommandResponse {
    pub result_string: String,
    pub ephemeral: bool,
}


#[derive(Eq , PartialEq, Debug)]
pub enum Rank {
    Admin,
    Mod,
    Casual,
    Conocide,
    AFK,
    Random,
    Wtf
}



pub fn get_rank(_ctx: &serenity::client::Context , member: serenity::model::guild::Member) -> (Rank , RoleId) {
    
    let mut max: Rank = Rank::Wtf;
    let mut rank_id: RoleId = RoleId::new(1000916334537887846);
    for rank in member.roles {
        if rank == 715046083000139856 {
            max = Rank::Admin;
            rank_id = RoleId::new(715046083000139856);
        } else if rank == 715044649529770024 {
            max = Rank::Mod;
            rank_id = RoleId::new(715044649529770024);
        } else if rank == 904949584894263317 {
            max = Rank::Casual;
            rank_id = RoleId::new(904949584894263317);
        } else if rank == 1001600782514262118 {
            max = Rank::Conocide;
            rank_id = RoleId::new(1001600782514262118);
        } else if rank == 1000915278298873906 {
            max = Rank::AFK;
            rank_id = RoleId::new(1000915278298873906);
        } else if rank == 1000916334537887846 {
            max = Rank::Random;
            rank_id = RoleId::new(1000916334537887846);
        }
    }

    (max , rank_id)
}


pub fn rank_to_string(rank: serenity::model::id::RoleId) -> String {
    

    let mut rank_as_string: String = "No rank".to_string();
    if rank == 715046083000139856 {
        rank_as_string = "ADMIN".to_string();
    } else if rank == 715044649529770024 {
        rank_as_string = "MOD".to_string();
    } else if rank == 904949584894263317 {
        rank_as_string = "CASUAL".to_string();
    } else if rank == 1001600782514262118 {
        rank_as_string = "CONOCIDE".to_string();
    } else if rank == 1000915278298873906 {
        rank_as_string = "AFK".to_string();
    } else if rank == 1000916334537887846 {
        rank_as_string = "RANDOM".to_string();
    }
    rank_as_string
}
pub enum ConnectingType {
    Ready,
    Resumed
}
pub async fn current_ip_handler(_bot: &crate::Handler, ctx: &serenity::client::Context, connecting_type: ConnectingType){
    
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
        match connecting_type {
            ConnectingType::Ready => log::info!("Connected! Ip untouched from previous connection."),
            ConnectingType::Resumed => {}
        }
    } else {
        let Ok(_) = std::fs::write(std::path::Path::new("current_ip") , current_ip.to_string()) else {
            log::error!("Couldn't write current ip to file.");
            return;
        };
        if let Err(x) = lucrecio_user.direct_message(&ctx, serenity::builder::CreateMessage::new().content(format!("IP changed! {}" , current_ip.to_string()))).await {
            log::error!("Couldn't send new IP to Lucrecio.\n{x}");
        }
        match connecting_type {
            ConnectingType::Ready => log::info!("Connected! New IP is: {}", current_ip),
            ConnectingType::Resumed => log::info!("IP change detected! New IP is: {}" , current_ip)
        }
    }
}

pub async fn update_domain_ip(){
    let mut easy = curl::easy::Easy::new();
    easy.url("https://njal.la/update/?h=keepitpg.xyz&k=wbjjrqnjj65qrojq&auto").unwrap();
    easy.perform().unwrap();
}