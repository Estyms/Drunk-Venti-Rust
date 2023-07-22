use linked_hash_map::LinkedHashMap;
use serenity::client::Context;
use serenity::model::application::interaction::application_command::{ApplicationCommandInteraction, CommandDataOption};
use serenity::model::application::interaction::InteractionResponseType;
use serenity::model::application::interaction::MessageFlags;
use serenity::model::application::interaction::message_component::MessageComponentInteraction;
use crate::data::artifacts::Artifact;
use crate::interactions::utils::create_action_row_basic;

pub async fn genshin_artifact_interaction(ctx: &Context, command: &ApplicationCommandInteraction, opt: &CommandDataOption) {

    let weapon = opt.options.get(0).expect("No argument for command Genshin artifact")
        .value.as_ref().expect("").as_str().expect("Not a string");

    let artifacts = Artifact::search(weapon).await;
    let mut artifact_list: LinkedHashMap<String, String> = LinkedHashMap::new();

    for a in artifacts {
        artifact_list.insert(a.id().to_string(), a.name().to_string());
    }

    let ar = create_action_row_basic(artifact_list, "artifact");

    command.create_interaction_response(&ctx.http, |res| {
        res.kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|d| {
                d.content("Select an Artifact")
                    .components(|c| c.add_action_row(ar))
                    .flags(MessageFlags::EPHEMERAL)
            })
    }).await.expect("Message didn't got sent");
}


pub async fn show_artifact_embed(ctx: &Context, command: &MessageComponentInteraction, artifact_name: String){
    let artifact = Artifact::get(artifact_name.as_str()).await;
    let embed = artifact.to_embed().await;
    command.create_interaction_response(&ctx.http, |res| {
        res.kind(InteractionResponseType::UpdateMessage)
            .interaction_response_data(|d| {
                d.set_embeds(vec!(embed).into_iter())
                    .flags(MessageFlags::EPHEMERAL)
            })
    }).await.expect("Interaction failed");
}