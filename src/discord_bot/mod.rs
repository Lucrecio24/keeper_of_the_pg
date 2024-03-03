pub mod message_handler;
pub mod interaction_handler;
pub mod new_member_handler;
pub mod updated_member_handler;
pub mod commands;
pub mod button_handler;



extern crate dotenvy;
use serenity::all::RoleId;

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
    
    /* APPLY CACHE PENDING
    if let Some(roles) = member.roles(){
        
    } else {
        member.add_roles(http, role_ids)
    }
     */

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
/*
pub fn simple_message(ctx: serenity::client::Context , channel_id:  serenity::model::id::ChannelId , message_content: String) => {
    let temp = channel_id.say(ctx , message)
    if let channel_id
}
*/