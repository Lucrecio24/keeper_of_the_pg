use keeper_of_the_pg::*;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    ApplicationCommandInteraction,
    CommandDataOptionValue
};

pub async fn run(
    command: &ApplicationCommandInteraction,
    ctx: &serenity::client::Context,
    database: &sqlx::SqlitePool
    ) -> CommandResponse {

    // CHECK IF ANY OPTION INSERTED, SHOULDN'T BE POSSIBLE
    if command.data.options.get(0).is_none() {
        println!("ERROR 101 INSULT COMMAND, discord sent no user...");
        CommandResponse{
            result_string: "ERROR 101".to_string(),
            ephemeral: true
        }
    } else {
        // SAVE OPTION AS OPTION
        let option = command.data.options.get(0)
        .expect("Expected user option")
        .resolved
        .as_ref()
        .expect("Expected user object");

        // Unwrapping option user for its user struct and member struct.
        if let CommandDataOptionValue::User(target_user, _target_member) = option {
            // The toty. If the target is the toty, we don't subtract lanas coin
            if target_user.id == 408088734722949132 {

                let target_id = target_user.clone().id.as_mut_u64().to_string();
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
                                let message_result = target_user.direct_message(&ctx.http, |message| message.content(format!("El toty se la come\nTe han insultado {} veces." , new_amount))).await;
                                match message_result {
                                    Ok(_x) => {
                                        CommandResponse{
                                            result_string: "Toty insultado por dms >:)".to_string(),
                                            ephemeral: false
                                        }
                                    }
                                    Err(_error) => {
                                        CommandResponse{
                                            result_string: "Error al enviar...".to_string(),
                                            ephemeral: true
                                        }
                                    }
                                }
                            }
                            Err(error) => {
                                println!("COULDN'T UPDATE DATABASE\n{}" , error);
                                CommandResponse {
                                    result_string: "No se pudo actualizar la base de datos".to_string(),
                                    ephemeral: true
                                }
                            }
                        }

                    }
                    Err(_) => {
                        CommandResponse{
                            result_string: "Error al enviar...".to_string(),
                            ephemeral: true
                        }
                    }
                }
            } else {
                CommandResponse {
                    result_string: "Comando no implementado para no-totys...".to_string(),
                    ephemeral: true
                }
            }
        } else {
            println!("ERROR 102 INSULT COMMAND, discord sent no user...");
            CommandResponse{
                result_string: "ERROR 102".to_string(),
                ephemeral: true
            }
        }
    }
}


pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("insult")
        .description("Insult someone via dms").create_option(|option| {
            option
                .name("user")
                .description("The user to insult")
                .kind(CommandOptionType::User)
                .required(true)
    })
}
