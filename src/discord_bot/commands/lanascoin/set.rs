use crate::discord_bot::*;
use serenity::all::{CommandInteraction , Mention};


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
    // Checking if member was inputted

    // Saving first option (member) under first_option variable, or returning if None
    let Some(target_user_id) = &command.data.options[0].value.as_user_id() else {
        return CommandResponse{
            result_string: "Miembro a buscar no ingresado".to_string(),
            ephemeral: true
        };
    };
    // Saving second option (member) under second_option variable, or returning if None
    let Some(new_amount) = &command.data.options[0].value.as_i64() else {
        return CommandResponse{
            result_string: "Nueva cantidad no ingresada".to_string(),
            ephemeral: true
        };
    };

    if new_amount < &0 {
        return CommandResponse{
            result_string: "Cantidad ingresada no valida. Numeros positivos porfa".to_string(),
            ephemeral: true
        };
    }

    let target_user_id = target_user_id.to_string();
    
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
            CommandResponse{
                result_string: format!("{} ahora tiene {}LanasCoins" , Mention::from(RoleId::from(target_user_id.parse::<u64>().unwrap())) ,&new_amount),
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




    //OLD COMMAND
    //Safely unwrapping second option
    /*
    let new_amount: i64;
    if let CommandDataOptionValue::Integer(value) = second_option {
        // Checking if value is positive, and if not, returning invalid amount.
        if value < &0 {
            return CommandResponse{
                result_string: "Amount value invalid".to_string(),
                ephemeral: true
            }
        }
        // Saving value as new_amount.
        new_amount = *value;
    } else {
        return CommandResponse{
            result_string: "Amount value invalid".to_string(),
            ephemeral: true
        }
    }
    //Safely unwrapping first option for its user and member
    if let CommandDataOptionValue::User(target_user, _target_member) = first_option {

        // Saving user id to make query. Then make, await and save query.
        // Querying for the current amount of lanas_coins the target user has.
        let target_user_id = target_user.id.as_u64().to_string();

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
                    
                    CommandResponse{
                        result_string: format!("{} ahora tiene {}LanasCoins" , Mention::from(target_user.id) ,&new_amount),
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
    } else {
        // If query didn't yield, inform user he's not in the database
        CommandResponse{
            result_string: "User not found in database...".to_string(),
            ephemeral: true
        }
    }
}
*/
