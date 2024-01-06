use crate::discord_bot::*;

use serenity::{
    builder::{CreateCommand , CreateCommandOption},
    model::application::CommandOptionType, 
    all::CommandInteraction
};

pub async fn run(
    command: &CommandInteraction,
    ctx: &serenity::client::Context,
    database: &sqlx::SqlitePool
    ) -> CommandResponse {

    // CHECK IF ANY OPTION INSERTED, SHOULDN'T BE POSSIBLE
    if command.data.options.get(0).is_none() {
        println!("ERROR 101 INSULT COMMAND, discord sent no user...");
        return CommandResponse{
            result_string: "ERROR 101".to_string(),
            ephemeral: true
        };
    }
    // SAVE OPTION AS OPTION
    let Some(option) = command.data.options.get(0) else {
        println!("ERROR 102 INSULT COMMAND, discord sent no user...");
        return CommandResponse{
            result_string: "ERROR 102".to_string(),
            ephemeral: true
        };
    };
    let Some(user) = option.value.as_user_id() else {
        println!("ERROR 102 INSULT COMMAND, discord sent no user...");
        return CommandResponse{
            result_string: "ERROR 102".to_string(),
            ephemeral: true
        };
    };

    match user.get() {
        //El toty
        408088734722949132 => {
            let target_id = user.clone().to_string();
            // Querying for the current amount of command uses
            let query_result = sqlx::query!(
                "SELECT amount FROM cumulative_command_data WHERE account_id = ?",
                target_id
            )
            .fetch_one(database)
            .await;
        
            match query_result {
                Ok(result) => {
                    let new_amount = result.amount + 1;
                    let update_query = sqlx::query!(
                        "UPDATE cumulative_command_data SET amount = ? WHERE account_id = ?",
                        new_amount,
                        target_id
                    )
                    .execute(database)
                    .await;


                    match update_query {
                        Ok(_) => {

                            let Ok(dm_channel) = user.create_dm_channel(ctx).await else {
                                return CommandResponse {
                                    result_string: "Couldn't connect to discord API".to_string(),
                                    ephemeral: true
                                };
                            };

                            let message_result = dm_channel.say(ctx, format!("El toty se la come\nTe han insultado {} veces." , new_amount)).await;
                            match message_result {
                                Ok(_x) => {
                                    return CommandResponse{
                                        result_string: "Toty insultado por dms >:)".to_string(),
                                        ephemeral: false
                                    };
                                }
                                Err(_error) => {
                                    return CommandResponse{
                                        result_string: "Error al enviar...".to_string(),
                                        ephemeral: true
                                    };
                                }
                            }
                        }
                        Err(error) => {
                            println!("COULDN'T UPDATE DATABASE\n{}" , error);
                            return CommandResponse {
                                result_string: "No se pudo actualizar la base de datos".to_string(),
                                ephemeral: true
                            };
                        }
                    }

                }
                Err(_) => {
                    return CommandResponse{
                        result_string: "Error al enviar...".to_string(),
                        ephemeral: true
                    };
                }
            }
        }
        569648467023102022 => {
            return CommandResponse {
                result_string: "NO INSULTAR AL JOSETO".to_string(),
                ephemeral: true
            };
        }
        _ => {
            return CommandResponse {
                result_string: "Comando no implementado para no-totys...".to_string(),
                ephemeral: true
            };
        }
    }
}


pub fn register() -> CreateCommand {
    
    CreateCommand::new("insult")
        .description("Insult someone via dms")
        .add_option(CreateCommandOption::new(
            CommandOptionType::User,
            "user",
            "The user to insult")
        .required(true))
}
