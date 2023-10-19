use std::time::Duration;
use rand::Rng;
use serenity::{
    client::{
        Context
    }
};
use serenity::builder::CreateEmbed;
use serenity::model::application::interaction::MessageFlags;
use serenity::model::id::{ChannelId, MessageId};
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;

use serenity::model::prelude::{InteractionResponseType};
use serenity::utils::Colour;
use crate::interactions::data::events::Event;
use crate::utils::mongo::{add_discord_status_message, get_all_status_messages, get_discord_status_message, StatusMessage};


pub async fn update_status_message(ctx: Context) {
    tokio::task::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(60*60));
        loop {
            let mut x = get_all_status_messages().await;
            x.reverse();
            let embeds = create_status_embed().await;
            for sm in x {
                let embeds : Vec<CreateEmbed> = embeds.clone();
                let ctx = ctx.clone();
                tokio::spawn( async move {
                    if sm.channel_id == 0 { return }
                    let msg = ChannelId::from(sm.channel_id as u64).message(ctx.clone().http, sm.message_id as u64).await;
                    match msg {
                        Ok(mut m) => {
                            match m.edit(&ctx.http, |f| {
                                f.set_embeds((*embeds).to_owned())
                            }).await {
                                Ok(_) => {},
                                Err(e) => println!("Error while editing message {}", e)
                            }
                        }
                        Err(_) => {
                            println!("Cannot update guild : {}", sm.channel_id);
                        }
                    }
                });
            };

            interval.tick().await;
        }
    });
}

#[allow(dead_code)]
pub async fn create_status_interaction(ctx: Context, command: ApplicationCommandInteraction) {
    let altcommand = command.clone();
    if !altcommand.member.unwrap().permissions.expect("No permissions").manage_messages() {
        command.create_interaction_response(&ctx.http, |f| {
            f.kind(InteractionResponseType::ChannelMessageWithSource).interaction_response_data(|r| {
                r.embed(|e| {
                    e.title("Command Unsuccessful").description("You do not have the right to manage messages.")
                        .color(Colour::from(0xff0000))
                }).flags(MessageFlags::EPHEMERAL)
            })
        }).await.expect("Interaction didn't work");
        return;
    }


    let embeds = create_status_embed().await;
    let channel_option = &command.data.resolved.channels;
    let channel_id = channel_option.keys().next().expect("No options passed");

    let message = get_discord_status_message(&command.guild_id.expect("Not in guild").as_u64().to_owned()).await;
    if let Some(e) = message {
        let rm = ChannelId::from(e.channel_id as u64)
            .message(&ctx.http, MessageId::from(e.message_id as u64))
            .await;
        match rm {
            msg if rm.is_ok() => {
                msg.unwrap().delete(&ctx.http).await.unwrap();
            }
            _ => {}
        }
    };

    let msg = channel_id.send_message(&ctx.http, |f| {
        f.add_embeds(embeds)
    }).await.expect("Can't send Message");

    let sm = StatusMessage {
        message_id: *msg.id.as_u64() as i64,
        channel_id: *msg.channel_id.as_u64() as i64,
        guild_id: *command.guild_id.expect("Not in guild").as_u64() as i64,
    };

    add_discord_status_message(sm).await;


    command.create_interaction_response(&ctx.http, |r| {
        r.kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|d| {
                d.embed(|e| {
                    e.title("Command Successful !");
                    e.color(Colour::new(0x00ff00));
                    e.description("Status message created !".to_string())
                })
                    .flags(MessageFlags::EPHEMERAL)
            })
    }).await.unwrap();
}

async fn create_status_embed() -> Vec<CreateEmbed> {
    let mut embeds: Vec<CreateEmbed> = vec![];

    let mut current = Event::get_current().await;
    let mut others = current.clone();

    let mut question_marks = String::new();

    let mut upcoming = Event::get_upcoming().await;

    current = current.into_iter().filter(|p| p.show_on_home.unwrap_or(false)).collect::<Vec<Event>>();
    others = others.into_iter().filter(|p| !p.show_on_home.unwrap_or(false)).collect::<Vec<Event>>();


    upcoming = upcoming.into_iter().filter(|p| p.show_on_home.unwrap_or(false)).collect::<Vec<Event>>();
    let upcoming_event: Option<&Event> = upcoming.get(0);

    for e in current {
        question_marks = format!("{}?", question_marks);
        let mut embed = CreateEmbed::default();
        embed.title(e.name);
        embed.color(Colour::new(rand::thread_rng().gen_range(0x000000..0xffffff)));

        if let Some(url) = &e.image {
            embed.image(format!("https://raw.githubusercontent.com/MadeBaruna/paimon-moe/main/static/images/events/{}", url.replace(' ', "%20")));
        };

        if let Some(t) = e.url { embed.url(format!("{}{}", t, question_marks)); };

        embed.description(format!("Ends : <t:{}:R>", e.end_timestamp.expect("No End Timestamp")));
        embeds.push(embed);
    }


// Other events embed
    let mut other_embed = CreateEmbed::default();
    other_embed.title("Other Events");
    other_embed.color(Colour::new(rand::thread_rng().gen_range(0x000000..0xffffff)));

    for e in others {
        other_embed.field(e.name, format!("Ends : <t:{}:R>", e.end_timestamp.expect("No End Timestamp")), false);
    }
    embeds.push(other_embed);

    if let Some(e) = upcoming_event {
        let mut upcoming_embed = CreateEmbed::default();
        question_marks = format!("{}?", question_marks);
        upcoming_embed.title(&e.name);
        upcoming_embed.description(format!("Starts : <t:{}:R>", e.start_timestamp.expect("No Start Timestamp")));
        upcoming_embed.color(Colour::new(rand::thread_rng().gen_range(0x000000..0xffffff)));

        if let Some(url) = &e.image {
            upcoming_embed.image(format!("https://github.com/MadeBaruna/paimon-moe/raw/main/static/images/events/{}", url.replace(' ', "%20")));
        };

        if let Some(url) = &e.url { upcoming_embed.url(format!("{}{}", url, question_marks)); };
        embeds.push(upcoming_embed);
    };

    embeds
}