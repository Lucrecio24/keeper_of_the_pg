use serenity::model::prelude::interaction::application_command::{
    ApplicationCommandInteraction,
    CommandDataOptionValue,
};
use serenity::model::mention::Mention;

use keeper_of_the_pg::*;



pub async fn run(
    ctx: &serenity::client::Context,
    command: &ApplicationCommandInteraction,
    database: &sqlx::SqlitePool)
     -> CommandResponse {

    match get_rank(ctx , command.member.as_ref().unwrap().clone()).0 {
        Rank::Admin => {
            //If admin we continue with the program
        }
        _ => {
            return CommandResponse{
            result_string: format!("No tienes suficiente rango para usar este comando."),
            ephemeral: true
            }
        }
    }
    // Checking if command data was inputted, and returning if none
    // Checking if member was inputted
    if command.data.options[0].options.get(0).is_none() {
        return CommandResponse{
            result_string: format!("Miembro a buscar no ingresado"),
            ephemeral: true
        }
    }
    // Checking if amount was inputted
    if command.data.options[0].options.get(1).is_none() {
        return CommandResponse{
            result_string: format!("Cantidad a sumar no ingresada"),
            ephemeral: true
        }
    }
    // Saving first option (member) under first_option variable
    let first_option = &command.data.options[0].options[0]
    .resolved
    .as_ref()
    .unwrap();
    // Saving second option (member) under second_option variable
    let second_option = &command.data.options[0].options[1]
    .resolved
    .as_ref()
    .unwrap();

    //Safely unwrapping second option
    let amount_to_add: u64;
    if let CommandDataOptionValue::Integer(value) = second_option {
        // Checking if value is positive, and if not, returning invalid amount.
        if value < &1 {
            return CommandResponse{
                result_string: format!("Amount value invalid"),
                ephemeral: true
            }
        }
        // Saving value as amount_to_add
        amount_to_add = *value as u64;
    } else {
        return CommandResponse{
            result_string: format!("Amount value invalid"),
            ephemeral: true
        }
    }

    //Safely unwrapping first option for its user and member
    if let CommandDataOptionValue::User(target_user, _target_member) = first_option {

        // Saving user id to make query. Then make, await and save query.
        // Querying for the current amount of lanas_coins the target user has.
        let target_user_id = target_user.id.as_u64().to_string();
        let query_result = sqlx::query!(
            "SELECT lanas_coin FROM members WHERE account_id = ?",
            target_user_id
        )
        .fetch_one(database)
        .await;

        // If the query gave something, print the result back, with the corresponding data
        if let Ok(result) = query_result {
            // Calculating new lanas_coin amount
            let new_amount: i64 = result.lanas_coin + amount_to_add as i64;


            // Updating database with new lanas_coin amount
            let update_query = sqlx::query!(
                "UPDATE members SET lanas_coin = ? WHERE account_id = ?",
                new_amount,
                target_user_id
            )
            .execute(database)
            .await;
            
            match update_query {
                // Returning successful result
                Ok(_) => {
                    
                    return CommandResponse{
                        result_string: format!("Se le han sumado {} LanasCoins a {}" , &amount_to_add , Mention::from(target_user.id.clone())),
                        ephemeral: true
                    }
                }
                // Returning failed result
                Err(error) => {
                    println!("{}" , error);
                    return CommandResponse{
                        result_string: format!("No se pudo actualizar la base de datos con los nuevos valores"),
                        ephemeral: true
                    }
                }
            }
        } else {
            // If query didn't yield, inform user he's not in the database
            return CommandResponse{
                result_string: format!("User not found in database..."),
                ephemeral: true
            }
        }
    } else {
            //If query didn't yield, inform user
        return CommandResponse{
            result_string: format!("User not found in database..."),
            ephemeral: true
        }
    }
}