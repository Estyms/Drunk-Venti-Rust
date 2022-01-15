use linked_hash_map::LinkedHashMap;
use serenity::client::Context;
use serenity::model::interactions::application_command::ApplicationCommandInteraction;
use serenity::model::interactions::application_command::ApplicationCommandInteractionDataOption;
use serenity::model::interactions::InteractionResponseType;
use serenity::model::prelude::InteractionApplicationCommandCallbackDataFlags;
use serenity::model::prelude::message_component::MessageComponentInteraction;
use crate::data::weapons::Weapon;
use crate::interactions::utils::create_action_row_basic;

pub async fn genshin_weapon_interaction(ctx: &Context, command: &ApplicationCommandInteraction, opt: &ApplicationCommandInteractionDataOption) {

    let weapon = opt.options.get(0).expect("No argument for command Genshin build")
        .value.as_ref().expect("").as_str().expect("Not a string");

    let weapons = Weapon::search(weapon).await;
    let mut weapon_list: LinkedHashMap<String, String> = LinkedHashMap::new();

    for w in weapons {
        weapon_list.insert(w.id().to_string(), w.name().to_string());
    }

    let ar = create_action_row_basic(weapon_list, "weapon");

    command.create_interaction_response(&ctx.http, |res| {
        res.kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|d| {
                d.content("Select a Weapon")
                    .components(|c| c.add_action_row(ar))
                    .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
            })
    }).await.expect("The response didn't get sent");
}


pub async fn show_weapon_embed(ctx: &Context, command: &MessageComponentInteraction, weapon_name: String){
    let weapon = Weapon::get(weapon_name.as_str()).await;
    command.create_interaction_response(&ctx.http, |res| {
        res.kind(InteractionResponseType::UpdateMessage)
            .interaction_response_data(|d| {
                d.embeds(vec!(weapon.to_embed()).into_iter())
                    .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
            })
    }).await.expect("The message didn't got sent");
}