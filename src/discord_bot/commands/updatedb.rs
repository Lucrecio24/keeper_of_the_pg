use crate::discord_bot::*;
use serenity::{
    builder::CreateCommand,
    all::CommandInteraction,
};
use serenity::futures::StreamExt;

pub async fn run(
    ctx: &serenity::client::Context,
    command: &CommandInteraction,
    database: &sqlx::SqlitePool
    ) -> CommandResponse {

    //OLD COMMAND

    if get_rank(ctx , *command.member.as_ref().unwrap().clone()).0 != Rank::Admin{
        return CommandResponse{
            result_string: "No tienes suficiente rango para usar este comando.".to_string(),
            ephemeral: true
        };
    }

        // Check if user is admin
    match get_rank(ctx , *command.member.as_ref().unwrap().clone()).0 {

        Rank::Admin => {
            //Get stream(?) of guild members and iterate through them
            let mut guild_members = command.guild_id.unwrap().members_iter(ctx).boxed();
            while let Some(member_result) = guild_members.next().await {
                let Ok(current_member) = &member_result else {
                    break
                };
                let member_id = &current_member.user.id.to_string();
                let member_rank_id = get_rank(ctx , member_result.as_ref().unwrap().clone()).1.to_string();


                let query_result = sqlx::query!(
                    "SELECT account_id FROM members WHERE account_id = ?",
                    member_id
                )
                .fetch_one(database)
                .await;


                if let Ok(_result) = query_result{
                    println!("Found in the database: {}" , member_result.as_ref().unwrap().user.name);
                } else {
                    sqlx::query!(
                        "INSERT INTO MEMBERS (account_id , rank_id , lanas_coin) VALUES (? , ? , ?)",
                        member_id,
                        member_rank_id,
                        0,
                    )
                    .execute(database)
                    .await
                    .unwrap();
                    println!("Inserted into the database: {}" , member_result.as_ref().unwrap().user.name)
                }
            }
            CommandResponse{
                result_string: "Database updated.".to_string(),
                ephemeral: true
            }
        }
        _ => {
            CommandResponse{
                result_string: "No tienes suficiente rango para usar este comando.".to_string(),
                ephemeral: true
            }
        }
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("updatedb")
        .description("Updates database with every member in guild")
}