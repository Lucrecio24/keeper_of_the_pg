use keeper_of_the_pg::*;
use serenity::builder::CreateApplicationCommand;
use serenity::model::mention::Mention;
use serenity::model::prelude::command::{CommandOptionType};
use serenity::model::prelude::interaction::application_command::{
    ApplicationCommandInteraction,
    CommandDataOptionValue,
};

pub async fn run(command: &ApplicationCommandInteraction , ctx: &serenity::client::Context) -> CommandResponse {
    
    // COMMAND STILL IN DEVELOPMENT, THIS CHANNEL CHECK HAS TO GO.
    if command.channel_id != 959921482551668756 {
        return CommandResponse{
            result_string: format!("Comando no implementado aún, paciencia porfa"),
            ephemeral: true
        }
    } else {
        //ACTUAL COMMAND STARTS HERE
        
        //SAVE COMMAND DATA OPTIONS TO ITS OWN VARIABLE AND CHECK IF IT IS NONE
        let options = &command.data.options;
        if options.get(0).is_none(){
            //IF NONE, NO OPTION WAS INPUTTED WHEN COMMAND WAS USED, AND IN THIS CASE, IT WORKS TO RESET NICKNAME
            command.guild_id.unwrap().edit_member(&ctx , command.user.id , |m| m.nickname("")).await.unwrap();
            return CommandResponse{
                result_string: format!("{} quiere quitarse su apodo" , Mention::from(command.user.id)),
                ephemeral: false
            }
        } else {
            //ELSE, IF COMMAND DATA WAS INPUTTED, WE CHANGE THE MEMBER'S NICK TO THE COMMAND DATA.
            //TODO: THIS SHOULD WORK AS A REACTION-BASED COMMAND, SO IT FIRST GIVES THE MESSAGE
            //      THAT THE USER WANTS TO CHANGE THE NICKNAME, WAITS FOR A ADMIN/MOD REACTION,
            //      AND THEN DOES THE CHANGE.
            if let CommandDataOptionValue::String(new_nickname) = options.get(0).expect("No command data option").resolved.as_ref().unwrap() {
                command.guild_id.unwrap().edit_member(&ctx , command.user.id , |m| m.nickname(new_nickname)).await.unwrap();

                return CommandResponse{
                    result_string: format!("{} quiere cambiar su apodo a {}" , Mention::from(command.user.id) , new_nickname),
                    ephemeral: false
                }
            } else {
                //THIS CLAUSE EXISTS IF, FOR SOME WEIRD REASON, THE COMMANDOPTIONTYPE DOESN'T RETURN A MEMBER STRUCT
                //NOT PROBABLE, BUT WHO KNOWS, COULD HAPPEN
                return CommandResponse{
                    result_string: format!("Invalid member struct, inform an Admin."),
                    ephemeral: true
                }
            }
        }
    }
}



pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("callme").description("Change nickname of user").create_option(|option| {
        option
            .name("nick")
            .description("New nickname")
            .kind(CommandOptionType::String)
            .required(false)
    })
}