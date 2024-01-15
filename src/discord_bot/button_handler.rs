use serenity::{
    all::RoleId,
    builder::{CreateInteractionResponse , CreateInteractionResponseMessage , CreateButton}
};

pub async fn role_message_builder() -> serenity::builder::EditMessage {
    let mut message = serenity::builder::EditMessage::new();
    message = message.content("Buena perkines, soy el Keeper\nClickeen el nombre del rol que quieren y se los doy\nEl toty se la come");
    message = message.button(CreateButton::new("rolebutton_juegolol").label("Juego lol"));
    message = message.button(CreateButton::new("rolebutton_lethalcompany").label("Lethal Company"));
    message = message.button(CreateButton::new("rolebutton_tarkov").label("Tarkov"));
    return message;
}

async fn toggle_role(ctx: &serenity::client::Context , member: serenity::all::Member , role: serenity::all::RoleId) -> Result<bool , &str> {
    let Some(roles) = member.roles(&ctx) else {
        return Err("BUTTON_HANDLER: No roles found");
    };
    let Some(role) = role.to_role_cached(ctx) else {
        return Err("BUTTON_HANDLER: Couldn't transform RoleId to Role");
    };


    if roles.contains(&role) {
        _ = member.remove_role(ctx, role).await;
        return Ok(false);
    } else {
        _ = member.add_role(ctx, role).await;
        return Ok(true);
    }
}

pub async fn rolebutton(ctx: serenity::client::Context, component: serenity::model::application::ComponentInteraction, second_word: &str) {
    let Some(member) = &component.member else {
        println!("BUTTON_HANDLER: No member found");
        return;
    };
    let Some(_roles) = member.roles(&ctx) else {
        println!("BUTTON_HANLDER: No roles found");
        return;
    };
    let mut response_message = CreateInteractionResponseMessage::new().ephemeral(true);
    let role_name: String;
    let role_id: RoleId;

    match second_word {
        "juegolol" => {
            role_name = "Juego lol".to_string();
            role_id = RoleId::new(991079843808575488);
        }
        "lethalcompany" => {
            role_name = "Lethal Company".to_string();
            role_id = RoleId::new(1179849933877149726);
        }
        "tarkov" => {
            role_name = "Tarkov".to_string();
            role_id = RoleId::new(904952184351895593);
        }
        _ => {
            role_name = "NO_NAME".to_string();
            role_id = RoleId::new(0);
            println!("BUTTON_HANDLER: second_word case not considered");
        }
    }
    if role_name != "NO_NAME".to_string(){
        match toggle_role(&ctx , member.clone() , role_id).await {
            Ok(x) => {
                match x {
                    true => {
                        response_message = response_message.content(format!("Ahora tienes el rol '{}'" , role_name));
                    }
                    false => {
                        response_message = response_message.content(format!("Te quité el rol '{}'" , role_name));
                    }
                }
            }
            Err(error) => {
                println!("{:?}" , error);
                response_message = response_message.content("No pude cambiar tus roles... Avisale al lucas pls");
            }
        }
    } else {
        response_message = response_message.content("Botón invalido. Avisale al lucas pls");
    }
    let response = CreateInteractionResponse::Message(response_message); 
    _ = component.create_response(&ctx, response).await;
}