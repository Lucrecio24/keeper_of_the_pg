pub mod id;
pub mod ip;
//pub mod test;
pub mod ping;
pub mod callme;
pub mod insult;
pub mod updatedb;
pub mod lanascoin;
pub mod server;

use serenity::{
    builder::CreateCommand,
    model::{
        application::Command,
        id::GuildId,
    },
};




pub async fn mass_registering(ctx: &serenity::client::Context) {
    use crate::discord_bot::commands as dbc;
    
    let guild_id = GuildId::from(234453296545267714);    
    let guild_command_list: Vec<CreateCommand> = vec![
        dbc::lanascoin::register(),
        dbc::server::register(),
        
        dbc::ping::register(),
        dbc::callme::register(),
        dbc::id::register(),
        dbc::insult::register(),
        dbc::updatedb::register()
    ];
    _ = guild_id.set_commands(&ctx, guild_command_list).await;


    let global_command_list: Vec<CreateCommand> = vec![
        dbc::ip::register()
    ];
    _ = Command::set_global_commands(&ctx , global_command_list).await;

}