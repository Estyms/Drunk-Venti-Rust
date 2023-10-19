mod interactions;
mod data;
pub mod utils;

use std::env;
use dotenv::dotenv;
use serenity::{
    async_trait,
    model::{
        gateway::Ready,
        application::{
            command::{
                Command,
                CommandOptionType,
            },
            interaction::{
                Interaction
            }
        },
    },
    prelude::*,
};
use serenity::model::gateway::Activity;
use serenity::model::prelude::OnlineStatus;
use crate::interactions::genshin::artifacts::show_artifact_embed;
use crate::interactions::genshin::build::build_interact;
use crate::interactions::genshin::weapons::show_weapon_embed;
use crate::interactions::status_message::update_status_message;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, _bot: Ready) {
        println!("{} connected!", _bot.user.name);
        update_status_message(_ctx.clone()).await;
        _ctx.set_presence(Some(Activity::playing("getting drunk with Kaeya")), OnlineStatus::Online).await;

        let x = Command::get_global_application_commands(&_ctx.http).await.unwrap();
        for y in &x {
            println!("{}", y.name);
        }
        for i in x {
            match i.name.as_str() {
                "genshin" => (),
                "createstatusmessage" => (),
                _ => {
                    let _result_delete = Command::delete_global_application_command(&_ctx.http, i.id).await;
                    match _result_delete {
                        Ok(()) => { println!("Deleted command {}", i.name) }
                        Err(f) => { println!("An error occurred deleting {} : {}", i.name, f) }
                    }
                }
            }
        }

        let x = Command::create_global_application_command(&_ctx.http, |command| {
            command.name("genshin").description("Get Informations about Genshin Impact.").create_option(|option| {
                option.name("builds")
                    .description("Shows builds for a Genshin Impact Characters")
                    .kind(CommandOptionType::SubCommand)
                    .create_sub_option(|so| {
                        so.name("character").description("Character to get builds for")
                            .kind(CommandOptionType::String)
                            .required(true)
                    })
            })
                .create_option(|option| {
                    option.name("weapon")
                        .description("Shows infos on a Genshin Impact weapon.")
                        .kind(CommandOptionType::SubCommand)
                        .create_sub_option(|so| {
                            so.name("name").description("Weapon you want infos on.")
                                .kind(CommandOptionType::String)
                                .required(true)
                        })
                })
                .create_option(|option| {
                    option.name("artifact")
                        .description("Shows infos on a Genshin Impact artifact set.")
                        .kind(CommandOptionType::SubCommand)
                        .create_sub_option(|so| {
                            so.name("artifact")
                                .description("Artifact Set you want infos on.")
                                .kind(CommandOptionType::String)
                                .required(true)
                        })
                })
        }).await;
        if x.is_err() {
            println!("{}", x.unwrap_err());
        }

        Command::create_global_application_command(&_ctx.http, |command| {
            command.name("createstatusmessage").create_option(|opt| {
                opt.name("channel").description("Channel where to put the status message")
                    .kind(CommandOptionType::Channel)
                    .required(true)
            }).description("Creates a status message of all the current events on Genshin Impact")
        }).await.expect("Can't create the createstatusmessage command");
    }


    async fn interaction_create(&self, _ctx: Context, _interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = _interaction {
            match command.data.name.as_str() {
                // Ping command
                "genshin" => interactions::genshin::genshin_interaction(_ctx, command).await,
                "createstatusmessage" => interactions::status_message::create_status_interaction(_ctx, command).await,
                // Unknown command
                _ => interactions::pong(_ctx, command).await
            }
        } else if let Interaction::MessageComponent(component) = _interaction {
            let mut args = component.data.custom_id.split('_');
            let command = args.next();
            match command.unwrap() {
                "weapon" => {
                    let weapon = args.collect::<Vec<_>>().join("_");
                    show_weapon_embed(&_ctx, &component, weapon).await;
                }
                "artifact" => {
                    let artifact = args.collect::<Vec<_>>().join("_");
                    show_artifact_embed(&_ctx, &component, artifact).await;
                }
                "build" => {
                    let character = args.collect::<Vec<_>>().join("_");
                    build_interact(&_ctx, &component, character).await;
                }
                _ => println!("Unknown interaction")
            }
        } else {
            println!("ERROR");
        }
    }
}

fn test_environment() {
    env::var("TOKEN").expect("TOKEN needed");
    env::var("MONGO_HOST").expect("MONGO_HOST needed");
    env::var("MONGO_PORT").expect("MONGO_PORT needed");
    env::var("API_HOST").expect("API_HOST needed");
    env::var("API_PORT").expect("API_PORT needed");
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    test_environment();
    let token= env::var("TOKEN").unwrap();
    let application_id: u64 = "860553396578811914".parse().expect("Wrong format");

    let needed_intents = [
        GatewayIntents::GUILDS,
        GatewayIntents::GUILD_MESSAGES,
        GatewayIntents::GUILD_EMOJIS_AND_STICKERS,
        GatewayIntents::GUILD_WEBHOOKS,
        GatewayIntents::GUILD_INTEGRATIONS
    ];

    let mut client = Client::builder(token, GatewayIntents::from_iter(needed_intents.into_iter()))
        .event_handler(Handler)
        .application_id(application_id)
        .await
        .expect("Error creating the client");

    if let Err(why) = client.start().await {
        println!("Client error {}", why);
    }
}
