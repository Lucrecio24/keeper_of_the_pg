use crate::discord_bot::*;
use public_ip::addr;
use tokio::fs::File;
use std::io::BufReader;
use std::io::BufRead;


use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;

pub async fn run(command: &ApplicationCommandInteraction , _ctx: &serenity::client::Context) -> CommandResponse {


    let tokio_file = match File::open("trusted").await.expect("WTF").try_into_std() {
        Ok(file) => {file}
        Err(_error) => {
            return CommandResponse{
                result_string: "Couldn't access trusted file...".to_string(),
                ephemeral: true
            }
        }
    };
    let trusted = BufReader::new(tokio_file).lines();

    let user = &command.user;
    let message_content = addr().await.unwrap();


    for line in trusted {
        if user.id.as_u64().to_string() == line.unwrap(){
            return CommandResponse{
                result_string: format!("Hi! Current ip is: {}" , message_content),
                ephemeral: true
            }
        }
    }
    CommandResponse{
        result_string: "No tienes permiso para usar este comando :(".to_string(),
        ephemeral: true
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("ip")
        .description("Enviarle ip al usuario")/*.create_option(|option| {
            option
                .name("id")
                .description("The user to lookup")
                .kind(CommandOptionType::User)
                .required(true) */
}
