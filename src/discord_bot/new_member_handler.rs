use serenity::all::Member;
pub async fn run(ctx: serenity::client::Context, new_member: Member){
    //Add new member to the RANDOM and RANDOMS role
    if let Err(x) = new_member.add_role(&ctx, 1000916334537887846).await{
        println!("Couldn't give RANDOM role to {}\nERROR: {x}" , new_member.user.name);
    }
    if let Err(x) = new_member.add_role(&ctx, 905161500359016498).await{
        println!("Couldn't give RANDOMS role to {}\nERROR: {x}" , new_member.user.name);
    }
    // Send message via Keep it PG's general chat
    if let Err(x) = serenity::model::id::ChannelId::from(234453296545267714).say(&ctx , format!("Bienvenido {} a este server ql malo borrenlo" , serenity::model::prelude::Mention::from(new_member.user.id))).await{
        println!("Couldn't send message to general chat after {} joined server.\nERORR: {x}" , new_member.user.name);
    }
}