use std::borrow::{Borrow};
use linked_hash_map::LinkedHashMap;
use serenity::builder::{CreateActionRow, CreateButton, CreateComponents, CreateEmbed};
use serenity::client::Context;
use serenity::model::application::component::ButtonStyle;
use serenity::model::application::interaction::{
    MessageFlags,
    InteractionResponseType,
    application_command::{
        ApplicationCommandInteraction,
        CommandDataOption
    },
    message_component::MessageComponentInteraction
};
use crate::data::artifacts::Artifact;
use crate::data::builds::{Builds, Role};
use crate::data::characters::Character;
use crate::data::weapons::Weapon;
use crate::interactions::utils::create_action_row_basic;

pub async fn genshin_build_interaction(ctx: &Context, command: &ApplicationCommandInteraction, opt: &CommandDataOption) {
    let char = opt.options.get(0).expect("No argument for command Genshin builds")
        .value.as_ref().expect("").as_str().expect("Not a string");

    let characters = Character::search(char).await;
    let mut character_list: LinkedHashMap<String, String> = LinkedHashMap::new();

    for c in characters {
        character_list.insert(c.id.to_string(), c.name.to_string());
    }

    let ar = create_action_row_basic(character_list, "build");

    command.create_interaction_response(&ctx.http, |res| {
        res.kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|d| {
                d.content("Select a Character")
                    .components(|c| c.add_action_row(ar))
                    .flags(MessageFlags::EPHEMERAL)
            })
    }).await.expect("Message didn't got sent");
}

fn is_menu(arg: &str) -> bool {
    matches!(arg, "home" | "artifacts" | "weapons" | "notes")
}

pub async fn build_interact(ctx: &Context, interaction: &MessageComponentInteraction, arguments: String) {
    let mut args = arguments.split('_');
    let command = args.next();


    match command.expect("") {
        c if is_menu(c) => {
            let role_index = args.next().expect("").parse::<usize>().expect("");
            let character_name = args.collect::<Vec<_>>().join("_");
            let character = Character::get(character_name.as_str()).await;
            let builds = Builds::get(character_name.as_str()).await;
            let role = builds.roles.get(role_index).expect("");
            let ar = nav_button(character_name, role_index);

            let embed = match c {
                "home" => home_embed(role, character).await,
                "artifacts" => artifact_embed(role, character).await,
                "weapons" => weapons_embed(role, character).await,
                "notes" => note_embed(role, character).await,
                _ => CreateEmbed::default()
            };

            interaction.create_interaction_response(&ctx.http, |res| {
                res.kind(InteractionResponseType::UpdateMessage)
                    .interaction_response_data(|d| {
                        d.add_embed(embed)
                            .components(|c| c.add_action_row(ar))
                    })
            }).await.expect("Can't send response");
        }
        _ => {
            let mut character = args.collect::<Vec<_>>().join("_");
            character = [command.expect(""), character.as_str()].join("_");
            character = character.strip_suffix('_').unwrap_or(character.as_str()).parse().unwrap();
            build_select(ctx, interaction, character).await;
        }
    }
}

fn nav_button(character_name: String, role_index: usize) -> CreateActionRow {
    let mut ar = CreateActionRow::default();

    let mut btn_home = CreateButton::default();
    btn_home
        .custom_id(format!("build_home_{}_{}", role_index, character_name))
        .style(ButtonStyle::Primary)
        .label("Home");

    let mut btn_artifacts = CreateButton::default();
    btn_artifacts
        .custom_id(format!("build_artifacts_{}_{}", role_index, character_name))
        .style(ButtonStyle::Primary)
        .label("Artifacts");

    let mut btn_weapons = CreateButton::default();
    btn_weapons
        .custom_id(format!("build_weapons_{}_{}", role_index, character_name))
        .style(ButtonStyle::Primary)
        .label("Weapons");

    let mut btn_notes = CreateButton::default();
    btn_notes
        .custom_id(format!("build_notes_{}_{}", role_index, character_name))
        .style(ButtonStyle::Primary)
        .label("Notes");

    let mut btn_builds = CreateButton::default();
    btn_builds
        .custom_id(format!("build_{}", character_name))
        .style(ButtonStyle::Success)
        .label("Other builds");

    ar.add_button(btn_home);
    ar.add_button(btn_artifacts);
    ar.add_button(btn_weapons);
    ar.add_button(btn_notes);
    ar.add_button(btn_builds);

    ar
}

async fn build_select(ctx: &Context, command: &MessageComponentInteraction, character: String) {
    let char = Character::get(character.as_str()).await;

    let roles = &char.builds;

    let roles = roles.iter().map(|c| c.name.to_string()).collect::<Vec<String>>();

    let mut rolemap = LinkedHashMap::<String, String>::new();
    for i in 0..roles.len() {
        rolemap.insert(format!("{}_{}", i, character), roles.get(i).expect("").to_string());
    }

    let ar = match roles.len() {
        0 =>  CreateActionRow::default(),
        _ =>create_action_row_basic(rolemap, "build_home")
    };

    command.create_interaction_response(&ctx.http, |res| {
        res.kind(InteractionResponseType::UpdateMessage)
            .interaction_response_data(|mut d| {
                if !roles.is_empty() {
                    d = d.components(|c| c.add_action_row(ar))
                } else {
                    d = d.set_components(CreateComponents::default())
                }
                d.embed(|mut e| {
                    e = e.title(format!("{}", char.name))
                        .thumbnail(format!("https://github.com/MadeBaruna/paimon-moe/raw/main/static/images/characters/{}.png", char.id))
                        .color(char.element.color)
                        .description("Select a build !")
                        .footer(|f| {
                            f.text(format!("Data from : https://paimon.moe/characters/{}", char.id))
                        });
                    if roles.is_empty() {
                        e = e.description("No builds available yet !")
                    }
                    e
                })
            })
    }).await.expect("Can't send response");
}


pub async fn home_embed(role: &Role, character: Character) -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed.title(format!("{} | {}", character.name, role.name));
    embed.thumbnail(format!("https://github.com/MadeBaruna/paimon-moe/raw/main/static/images/characters/{}.png", character.id));
    embed.color(character.element.color);

    match role.weapons.get(0) {
        Some(t) => {
            embed.field("Best Weapon",
                        Weapon::get(t.id.borrow()).await.name,
                        true,
            );
        }
        _ => {
            embed.field("Best Weapon", "TBD", true);
        }
    }

    embed.field("Skill Order",
                role.talent.iter().cloned().map(|t| format!("- {}\n", t))
                    .collect::<Vec<String>>().join("").strip_suffix('\n').expect(""),
                true,
    );


    match role.artifacts.get(0) {
        Some(a) => {
            embed.field("Best Artifacts", get_artifact_string(a).await, false);
        }
        _ => { embed.field("Best Artifacts", "TBD", false); }
    }

    embed.field("Circlet", &role.main_stats.circlet[0], true);
    embed.field("Goblet", &role.main_stats.goblet[0], true);
    embed.field("Sands", &role.main_stats.sands[0], true);

    embed.footer(|f| {
        f.text(format!("Data from : https://paimon.moe/characters/{}", character.id))
    });

    embed
}


async fn artifact_embed(role: &Role, character: Character) -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed.title(format!("{} | {}", character.name, role.name));

    match role.artifacts.get(0) {
        Some(t) if t.get(0).expect("").to_string() != "TBD" => {
            embed.thumbnail(format!("https://raw.githubusercontent.com/MadeBaruna/paimon-moe/main/static/images/artifacts/{}_circlet.png", t.get(0).expect("")));
        }

        _ => {
            embed.thumbnail(format!("https://github.com/MadeBaruna/paimon-moe/raw/main/static/images/characters/{}.png", character.id));
        }
    }

    embed.color(character.element.color);

    let mut all_artefacts_string: Vec<String> = vec![];
    for i in 0..role.artifacts.len() {
        match role.artifacts.get(i) {
            Some(a) => {
                all_artefacts_string.push(format!("- {}", get_artifact_string(a).await));
            }
            _ => { all_artefacts_string.push("- TBD".to_string()); }
        }
    }
    if !all_artefacts_string.is_empty() {
        embed.field("Best Artifacts", all_artefacts_string.join("\n"), false);
    } else {
        embed.field("Best Artifacts", "- TBD", false);
    }

    embed.field("Sub-stats", {
        format!("- {}", role.sub_stats.join("\n- "))
    }, false);

    embed.field("Circlet", &role.main_stats.circlet[0], true);
    embed.field("Goblet", &role.main_stats.goblet[0], true);
    embed.field("Sands", &role.main_stats.sands[0], true);

    embed.footer(|f| {
        f.text(format!("Data from : https://paimon.moe/characters/{}", character.id))
    });

    embed
}

async fn get_artifact_string(artifacts: &Vec<Box<str>>) -> String {
    let mut artifact_string: Vec<String> = vec![];
    let x = artifacts.len();
    for art in artifacts {
        let artifact_name = match art.to_string().as_str() {
            "+18%_atk_set" => "+18% Atk Set".to_string(),
            "+20%_hp_set" => "+20% Hp Set".to_string(),
            "+80_em" => "Elemental Mastery +80 Set".to_string(),
            t => Artifact::get(t).await.name.to_string(),
        };
        artifact_string.push(format!("({}) {}", match x {
            1 => 4,
            _ => 2
        }, artifact_name));
    }

    artifact_string.join(
        match x {
            1 | 2 => " & ",
            _ => ", "
        }
    )
}

async fn weapons_embed(role: &Role, character: Character) -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed.title(format!("{} | {}", character.name, role.name));

    match role.weapons.get(0) {
        Some(t) if t.id.to_string() != "TBD" => {
            embed.thumbnail(format!("https://raw.githubusercontent.com/MadeBaruna/paimon-moe/main/static/images/weapons/{}.png", t.id));
        }

        _ => {
            embed.thumbnail(format!("https://github.com/MadeBaruna/paimon-moe/raw/main/static/images/characters/{}.png", character.id));
        }
    }

    embed.color(character.element.color);

    let mut all_weapons_string: Vec<String> = vec![];
    for i in 0..role.weapons.len() {
        match role.weapons.get(i) {
            Some(a) if a.id.to_string() != "TBD" => {
                let weapon = Weapon::get(a.id.borrow()).await;
                all_weapons_string.push(format!("- {}", weapon.name));
            }
            _ => { all_weapons_string.push("- TBD".to_string()); }
        }
    }
    if !all_weapons_string.is_empty() {
        embed.field("Best Weapons", all_weapons_string.join("\n"), false);
    } else {
        embed.field("Best Weapons", "- TBD", false);
    }

    embed.footer(|f| {
        f.text(format!("Data from : https://paimon.moe/characters/{}", character.id))
    });


    embed
}

async fn note_embed(role: &Role, character: Character) -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed.title(format!("{} | {}", character.name, &role.name));
    embed.thumbnail(format!("https://github.com/MadeBaruna/paimon-moe/raw/main/static/images/characters/{}.png", character.id));
    embed.color(character.element.color);

    let n = &role.note.replace("<b>", "").replace("</b>", "");
    {
        let x = n.split('\n');
        let mut first = true;
        let mut add_before = "";
        for note_paragraph in x.collect::<Vec<&str>>() {
            if note_paragraph.is_empty() {continue};
            if note_paragraph.len() < 64 && add_before.is_empty(){
                add_before = note_paragraph;
                continue;
            }
            if !add_before.is_empty() {
                let note_paragraph = format!("**{}**\n{}", add_before, note_paragraph);
                embed.field(if first { "Notes" } else { "\u{200b}" }, note_paragraph, false);
                add_before = "";
            } else {
                embed.field(if first { "Notes" } else { "\u{200b}" }, note_paragraph, false);
                first = false;
            }

        }
    };

    let a = role.tip.as_ref();
    {
        let y = match a {
            None => { String::from("")}
            Some(b) => { b.to_string().replace("<b>", "").replace("</b>", "") }
        };
        let x = y.split('\n');
        let mut first = true;
        let mut add_before = "";
        for tip_paragraph in x.collect::<Vec<&str>>() {
            if tip_paragraph.is_empty() {continue};
            if tip_paragraph.len() < 64 {
                add_before = tip_paragraph;
                continue;
            }
            if !add_before.is_empty() {
                let tip_paragraph = format!("**{}**\n{}", add_before, tip_paragraph);
                embed.field(if first { "Tips" } else { "\u{200b}" }, tip_paragraph, false);
                add_before = "";
            } else {
                embed.field(if first { "Tips" } else { "\u{200b}" }, tip_paragraph, false);
                first = false;
            }

        }
    };



    embed.footer(|f| {
        f.text(format!("Data from : https://paimon.moe/characters/{}", character.id))
    });

    embed
}