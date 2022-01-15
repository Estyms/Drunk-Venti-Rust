pub mod status_message;
pub mod genshin;

#[path = "../data/mod.rs"]
mod data;
#[path = "../utils/mod.rs"]
pub mod utils;



use serenity::client::Context;
use serenity::model::interactions::{InteractionApplicationCommandCallbackDataFlags, InteractionResponseType};
use serenity::model::interactions::application_command::ApplicationCommandInteraction;

pub async fn pong(ctx : Context, command : ApplicationCommandInteraction) {
    let res = command.create_interaction_response(ctx.http, |res| {
        res.kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|response| {
                response.flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
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