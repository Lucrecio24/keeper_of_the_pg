
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::{
    ApplicationCommandInteraction,
};
use serenity::futures::StreamExt;


use keeper_of_the_pg::*;

pub async fn run(
    ctx: &serenity::client::Context,
    command: &ApplicationCommandInteraction,
    database: &sqlx::SqlitePool
    ) -> CommandResponse {

        // Check if user is admin
    match get_rank(ctx , command.member.as_ref().unwrap().clone()).0 {
        Rank::Admin => {
            //Get stream(?) of guild members and iterate through them
            let mut guild_members = command.guild_id.unwrap().members_iter(&ctx).boxed();
            while let Some(member_result) = guild_members.next().await {
                let member_id = &member_result.as_ref().unwrap().user.id.as_u64().to_string();
                let member_rank_id = keeper_of_the_pg::get_rank(ctx , member_result.as_ref().unwrap().clone()).1.as_u64().to_string();


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

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("updatedb")
        .description("Updates database with every member in guild")/*.create_option(|option| {
            option
                .name("id")
                .description("The user to lookup")
                .kind(CommandOptionType::User)
                .required(true)
    }) */
}