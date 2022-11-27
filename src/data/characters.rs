use std::env;
use reqwest::Url;
use serde_derive::{Serialize, Deserialize};
use crate::data::builds::{Role};
use crate::data::elements::Element;
use crate::data::items::Item;
use crate::data::shared_structs::{Ascension, WeaponType};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CharacterStats {
    pub hp: u64,
    pub atk: u64,
    pub def: u64
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CharacterMaterials {
    pub book: Vec<Item>,
    pub material: Vec<Item>,
    pub boss: Item
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Character {
    pub id: Box<str>,
    pub name: Box<str>,
    pub rarity: u8,
    pub weapon: WeaponType,
    pub stats: CharacterStats,
    pub ascension: Vec<Ascension>,
    pub element: Element,
    pub material: CharacterMaterials,
    pub builds: Vec<Role>
}

impl Clone for Character {
    fn clone(&self) -> Self {
        let x = serde_json::to_string(self).expect("str");
        serde_json::from_str::<Character>(x.as_str()).expect("An error occurred")
    }
}


impl Character {
    #[allow(dead_code)]
    pub(crate) async fn get(character: &str) -> Character {
        let host = env::var("API_HOST").unwrap();
        let port = env::var("API_PORT").unwrap();
        let url = format!("http://{}:{}/api/characters/{}", host, port, character);
        let url = Url::parse(&*url).expect("Can't convert url");
        let a  = reqwest::get(url.clone()).await.expect("Can't access Url");
        return reqwest::get(url).await.expect("Can't access Url").json::<Character>().await.expect("Wrong json format");
    }

    #[allow(dead_code)]
    async fn get_all() -> Vec<Box<str>> {
        let host = env::var("API_HOST").unwrap();
        let port = env::var("API_PORT").unwrap();
        let url = format!("http://{}:{}/api/characters", host, port);
        let url = Url::parse(&*url).expect("Can't convert url");
        return reqwest::get(url).await.expect("Can't access Url").json::<Vec<Box<str>>>().await.expect("Wrong json format");
    }

    #[allow(dead_code)]
    pub(crate) async fn search(character: &str) -> Vec<Character> {
        let host = env::var("API_HOST").unwrap();
        let port = env::var("API_PORT").unwrap();
        let url = format!("http://{}:{}/api/characters/search/{}", host, port, character);
        let url = Url::parse(&*url).expect("Can't convert url");
        return reqwest::get(url).await.expect("Can't access Url").json::<Vec<Character>>().await.expect("Wrong json format");
    }
}

#[test]
fn test_character() {
    let data = std::fs::read_to_string("test/character.json").expect("No character test file");
    serde_json::from_str::<Character>(&data).expect("Didn't work");
}