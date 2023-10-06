use keeper_of_the_pg::*;
use mc_query::query::stat_full;

//use serenity::builder::CreateApplicationCommand;
//use serenity::model::prelude::command::CommandOptionType;
//use serenity::model::prelude::{interaction::application_command::ApplicationCommandInteraction, command::Command};

pub async fn run(
    _ctx: &serenity::client::Context,
    mc_ip: &String,
    mc_port: &u16)
     -> CommandResponse {

    let full_data = stat_full(mc_ip.as_str() , *mc_port).await;
    match full_data {
        Ok(data) => {
            if data.players.len() == 0 {
                return CommandResponse{
                    result_string: String::from("Nobody is connected"),
                    ephemeral: true
                }
            }
            let mut player_list:String = String::from("");
            for player in data.players {
                player_list.push_str("-");
                player_list.push_str(&player);
                player_list.push_str("\n");
            }
            return CommandResponse{
                result_string: format!("Currently {} players online.\n{}" , data.num_players , player_list),
                ephemeral: true
            }
        }
        Err(error) => {
            return CommandResponse{
                result_string: format!("Couldn't connect to server.\n{}" , error),
                ephemeral: true
            }
        }
    }
    


    /*return CommandResponse{
        result_string: String::from("ERROR 101;\nEnd of the code."),
        ephemeral: true
    }*/
}
