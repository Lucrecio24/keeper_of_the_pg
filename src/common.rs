

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

pub fn getRank(ctx: &serenity::client::Context , member: serenity::model::guild::Member) -> Rank {
    
    /* APPLY CACHE PENDING
    if let Some(roles) = member.roles(){
        
    } else {
        member.add_roles(http, role_ids)
    }
     */

    let mut max: Rank = Rank::Wtf;
    for rank in member.roles {
        if rank == 715046083000139856 {
            max = Rank::Admin
        } else if rank == 715044649529770024 {
            max = Rank::Mod
        } else if rank == 904949584894263317 {
            max = Rank::Casual
        } else if rank == 1001600782514262118 {
            max = Rank::Conocide
        } else if rank == 1000915278298873906 {
            max = Rank::AFK
        } else if rank == 1000916334537887846 {
            max = Rank::Random
        }
    }

    return max
}