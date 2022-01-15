use serenity::client::Context;
use serenity::model::interactions::application_command::ApplicationCommandInteraction;
use crate::interactions::genshin::artifacts::genshin_artifact_interaction;
use crate::interactions::genshin::build::genshin_build_interaction;
use crate::interactions::genshin::weapons::genshin_weapon_interaction;

pub mod build;
pub mod weapons;
pub mod artifacts;



pub async fn genshin_interaction(ctx: Context, command: ApplicationCommandInteraction) {
    let sub_command = command.data.options.get(0).expect("No command provided");
    match sub_command.name.as_str() {
        "builds" => genshin_build_interaction(&ctx, &command, sub_command).await,
        "artifact" => genshin_artifact_interaction(&ctx, &command, sub_command).await,
        "weapon" => genshin_weapon_interaction(&ctx, &command, sub_command).await,
        _ => println!("Unknown Command")
    }
}