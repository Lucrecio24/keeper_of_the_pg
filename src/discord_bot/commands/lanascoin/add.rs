use crate::discord_bot::*;
use serenity::all::{CommandInteraction , Mention , UserId};

pub async fn run(
    ctx: &serenity::client::Context,
    command: &CommandInteraction,
    database: &sqlx::SqlitePool)
     -> CommandResponse {

    match get_rank(ctx , *command.member.as_ref().unwrap().clone()).0 {
        Rank::Admin => {
            //If admin we continue with the program
        }
        _ => {
            return CommandResponse{
            result_string: "No tienes suficiente rango para usar este comando.".to_string(),
            ephemeral: true
            }
        }
    }
    // Checking if command data was inputted, and returning if none

    // Saving first option (member) under first_option variable, or returning if None
    let Some(target_user_id) = &command.data.options[0].value.as_user_id() else {
        return CommandResponse{
            result_string: "Miembro a buscar no ingresado".to_string(),
            ephemeral: true
        };
    };
    // Saving second option (member) under second_option variable, or returning if None
    let Some(amount_to_add) = &command.data.options[0].value.as_i64() else {
        return CommandResponse{
            result_string: "Cantidad a sumar no ingresada".to_string(),
            ephemeral: true
        };
    };
    // Checking if number is positive and returning if not (ITS A ####ING INTEGER)
    if amount_to_add <&1 {
        return CommandResponse{
            result_string: "Amount value invalid".to_string(),
            ephemeral: true
        };
    }
    
    // Saving user id to make query. Then make, await and save query.
    // Querying for the current amount of lanas_coins the target user has.
    let target_user_id = target_user_id.to_string();
    let query_result = sqlx::query!(
        "SELECT lanas_coin FROM members WHERE account_id = ?",
        target_user_id
    ).fetch_one(database).await;

    let Ok(query_result) = query_result else {
        // If query didn't yield, inform user he's not in the database
        return CommandResponse{
            result_string: "User not found in database...".to_string(),
            ephemeral: true
        };
    };

    // Calculating new lanas_coin amount
    let new_amount: i64 = query_result.lanas_coin + *amount_to_add as i64;

    // Updating database with new lanas_coin amount
    let update_query = sqlx::query!(
        "UPDATE members SET lanas_coin = ? WHERE account_id = ?",
        new_amount,
        target_user_id
    ).execute(database).await;
    match update_query {
        // Returning successful result
        Ok(_) => {
            CommandResponse{
                result_string: format!("Se le han sumado {} LanasCoins a {}" , &amount_to_add , Mention::from(UserId::from(target_user_id.parse::<u64>().unwrap()))),
                ephemeral: true
            }
        }
        // Returning failed result
        Err(error) => {
            println!("{}" , error);
            CommandResponse{
                result_string: "No se pudo actualizar la base de datos con los nuevos valores".to_string(),
                ephemeral: true
            }
        }
    }
}