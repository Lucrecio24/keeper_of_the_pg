use serenity::model::id::GuildId;
use serenity::all::MemberAction;
use serenity::all::Change;
use serenity::model::guild::audit_log::Action;
use serenity::all::Mention;
use serenity::all::UserId;
pub async fn run(ctx: serenity::client::Context , entry: serenity::model::guild::audit_log::AuditLogEntry , guild_id: GuildId){

    if guild_id.get() != 234453296545267714 {
        println!("Audit log created outside KIPG. wtf.");
        return;
    }
    // Check if audit log was caused by the bot itself
    if entry.user_id.get() == 1004145386887319692 {
        return;
    }
    match entry.action {
        Action::Member(member_action) => {
            match member_action {
                // If action is an Update
                MemberAction::Update => {

                    let Some(changes_vector) = entry.changes else {
                        println!("ERROR: No changes done.");
                        return;
                    };
                    let Some(change) = changes_vector.get(0) else {
                        println!("");
                        return;
                    };
                    match change {
                        Change::Nick { old, new } => {
                            let second_line: String;
                            if old.is_none() & new.is_some() {
                                second_line = format!("Ahora se llama {}" , new.clone().unwrap_or(String::from("PLACEHOLDER")));
                            } else if old.is_some() & new.is_none() {
                                second_line = format!("Ya no tiene nick");
                            } else {
                                second_line = format!("{} :arrow_right: {}" , old.clone().unwrap_or(String::from("PLACEHOLDER")) , new.clone().unwrap_or(String::from("PLACEHOLDER")));
                            }
                            let first_line: String;
                            if entry.user_id.get() == entry.target_id.unwrap().get() {
                                first_line = format!("{} cambió su propio nick" , Mention::from(entry.user_id));
                            } else {
                                first_line = format!("{} cambió el nick de {}" , Mention::from(entry.user_id) , Mention::from(UserId::from(entry.target_id.unwrap().get())));
                            }
                            
                            let message = format!("{}\n{}" , first_line , second_line);
//                            println!("{}" , message);
                            let _ = serenity::model::id::ChannelId::from(1087524950425997383).say(ctx , message).await;
                        }
                        _ => {}
                    }
                }
                //If member action isn't one stated above, do nothing
                _ => {}
            }
        }

        // If action isnt one stated above, do nothing
        _ => {}
    }
}