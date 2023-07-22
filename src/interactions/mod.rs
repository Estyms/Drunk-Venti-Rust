pub mod status_message;
pub mod genshin;

use crate::data;
use crate::utils;



use serenity::client::Context;
use serenity::model::application::interaction::{MessageFlags, InteractionResponseType, application_command::ApplicationCommandInteraction};

pub async fn pong(ctx : Context, command : ApplicationCommandInteraction) {
    let res = command.create_interaction_response(ctx.http, |res| {
        res.kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|response| {
                response.flags(MessageFlags::EPHEMERAL)
                    .content("An error has occurred")
            })
    }).await;

    match res {
        Ok(()) => {}
        Err(e) =>{
            println!("An error has occured : {}", e)
        }
    }
}