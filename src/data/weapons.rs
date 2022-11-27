use std::env;
use reqwest::Url;
use serde_derive::{Serialize, Deserialize};
use serenity::builder::CreateEmbed;
use regex::Regex;
use crate::data::shared_structs::Ascension;
use crate::data::shared_structs::WeaponType;


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Skill {
    pub name: Option<Box<str>>,
    pub description: Option<Box<str>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Secondary {
    pub name: Option<Box<str>>,
    pub stats: Option<Vec<Option<f64>>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Extras {
    pub id: Box<str>,
    pub name: Box<str>,
    #[serde(rename = "type")]
    pub weapon_type: Box<str>,
    pub rarity: u8,
    pub description: Box<str>,
    pub skill: Skill,
    pub secondary: Secondary,
    pub atk: Vec<Option<f64>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Weapon {
    pub name: Box<str>,
    pub id: Box<str>,
    pub rarity: u8,
    pub atk: u64,
    pub secondary: Box<str>,
    #[serde(rename = "type")]
    pub weapon_type: WeaponType,
    pub source: Box<str>,
    pub ascension: Vec<Ascension>,
    pub extras: Extras,
}

impl Weapon {
    #[allow(dead_code)]
    pub async fn get(weapon: &str) -> Weapon {
        let host = env::var("API_HOST").unwrap();
        let port = env::var("API_PORT").unwrap();
        let url = format!("http://{}:{}/api/weapons/{}", host, port, weapon);
        let url = Url::parse(&*url).expect("Can't convert url");
        return reqwest::get(url).await.expect("Can't access Url").json::<Weapon>().await.expect("Wrong json format");
    }

    #[allow(dead_code)]
    pub async fn get_all() -> Vec<Box<str>> {
        let host = env::var("API_HOST").unwrap();
        let port = env::var("API_PORT").unwrap();
        let url = format!("http://{}:{}/api/weapons", host, port);
        let url = Url::parse(&*url).expect("Can't convert url");
        return reqwest::get(url).await.expect("Can't access Url").json::<Vec<Box<str>>>().await.expect("Wrong json format");
    }

    #[allow(dead_code)]
    pub async fn search(weapon: &str) -> Vec<Weapon> {
        let host = env::var("API_HOST").unwrap();
        let port = env::var("API_PORT").unwrap();
        let url = format!("http://{}:{}/api/weapons/search/{}", host, port, weapon);
        let url = Url::parse(&*url).expect("Can't convert url");
        return reqwest::get(url).await.expect("Can't access Url").json::<Vec<Weapon>>().await.expect("Wrong json format");
    }

    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        return &self.name;
    }

    #[allow(dead_code)]
    pub fn id(&self) -> &str {
        return &self.id;
    }

    #[allow(dead_code)]
    pub fn to_embed(&self) -> CreateEmbed {
        let mut embed = CreateEmbed::default();
        embed.title(format!("{} | {}", self.name, ":star:".repeat(self.rarity as usize)));

        embed.description(format!("{}", self.extras.description));

        embed.thumbnail(format!("https://raw.githubusercontent.com/MadeBaruna/paimon-moe/main/static/images/weapons/{}.png", self.id));

        // Fields
        match self.extras.skill.name.as_ref() {
            Some(t) => {
                let re = Regex::new("(<|</)span(?: [^>]*)?>").expect("Unknown regex");
                embed.field(format!("Passive : {}", t),
                            format!("{}",
                                    re.replace_all(self.extras.skill.description.as_ref()
                                                       .unwrap_or(&Box::from(""))
                                                       .to_string().as_str(), "**")).replace("\\n","\n"),
                            false);
            }
            _ => {}
        }
        embed.field("Main Stat", format!("{}", self.secondary), true);
        embed.field("Source", format!("{}", self.source), true);
        embed.field("\u{200b}", "\u{200b}", true);
        embed.field("Type", format!("{}", self.weapon_type.name), true);
        embed.field("Ascension Item", format!("{}", self.ascension.get(0).expect("").items.get(0).expect("").item.name), true);
        embed.field("\u{200b}", "\u{200b}", true);
        return embed;
    }
}

#[test]
fn test_weapon() {
    let data = std::fs::read_to_string("test/weapon.json").expect("No Weapon test file");
    serde_json::from_str::<Weapon>(&data).expect("Didn't work");
}