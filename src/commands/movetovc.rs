//WIP//

use keeper_of_the_pg::*;
use serenity::{
    builder::CreateApplicationCommand,
    model::{
        prelude::{
            command::CommandOptionType,
            ChannelType::Voice,
            interaction::application_command::{
                ApplicationCommandInteraction,
                CommandDataOptionValue
            }
        },
        application::{
            //command::Command,
            interaction::{InteractionResponseType}
        }
    }
};

pub async fn run(
    command: &ApplicationCommandInteraction,
    ctx: &serenity::client::Context,
    database: &sqlx::SqlitePool
    ) -> Result< () , CommandResponse> {

    // Command cost in lanascoins
    let mut movetovc_cost: i64 = 10;


    // Check if options were sent by discord. Shouldn't happen.
    if command.data.options.get(0).is_none() {
        println!("ERROR 101 INSULT COMMAND, discord sent no user...");
        return Err(CommandResponse{
            result_string: "ERROR 101".to_string(),
            ephemeral: true,
            show: true
        })
    }
    // Saving first option as first_option. Safe expect as the previous if handled anything else.
    let first_option = command.data.options.get(0)
    .expect("Expected user option")
    .resolved
    .as_ref()
    .expect("Expected user object");

    // Getting the user out of the OptionValue
    let target_user;
    match first_option {
        CommandDataOptionValue::User(user, _member) => {
            target_user = user;
        }
        _ => {
            // Happens if option sent by discord isn't a user
            return Err(CommandResponse{
                result_string: "ERROR 102".to_string(),
                ephemeral: true,
                show: true
            })
        }
    }
    let actioneer_user_id: String = command.user.id.0.to_string();
    // Quering for the amount of lanas_coin the actioneer user has.
    let lc_query = sqlx::query!(
        "SELECT lanas_coin FROM members WHERE account_id = ?",
        actioneer_user_id
    )
    .fetch_one(database)
    .await;
    //Checking if actioneer user has enough lanas_coin to use command.
    match lc_query {
        Ok(result) => {
            // If the target is The Toty, we do a 50% discount.
            if target_user.id.0 == 408088734722949132 {
                movetovc_cost = movetovc_cost/2;
            }
            // We check if amount is enough, else we return with message.
            if result.lanas_coin < movetovc_cost {
                return Err(CommandResponse{
                    result_string: format!("Not enough LanasCoins!\nYou need {} LanasCoins to use this command" , movetovc_cost),
                    ephemeral: true,
                    show: true
                })
            }
        }
        Err(_error) => {
            return Err(CommandResponse{
                result_string: "Not in the database... Inform an admin please :)".to_string(),
                ephemeral: true,
                show: true
            })
        }
    }

    //Checking if toty.
    match target_user.id.0 {
        408088734722949132 => {println!("sexologia marina")}
        _ => {
            
        }
    }
    let first_response = command.create_interaction_response(ctx, |response|
        response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|message| 
                message
                .content("This will cost your soul")
                .ephemeral(true))
    ).await;
    let second_response = command.create_followup_message(ctx, |message|
        message
            .content("And it won't be pretty")
            .ephemeral(true)
    ).await;

    println!("First response: {:?}\n\nSecond response: {:?}" , first_response , second_response);

    return Err(CommandResponse{
        result_string: "Command finished succesfully".to_string(),
        ephemeral: true,
        show: true
    })
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("movetovc")
        .description("Isolate someone on a voicechat")
        .create_option(|option| {
            option
                .name("user")
                .description("user to move")
                .kind(CommandOptionType::User)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("channel")
                .description("target channel")
                .kind(CommandOptionType::Channel)
                .channel_types(&[Voice])
                .required(true)
        })
}
