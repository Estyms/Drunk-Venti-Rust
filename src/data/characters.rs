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
    pub name: Box<str>,
    pub id: Box<str>,
    pub rarity: u8,
    pub element: Element,
    pub weapon: WeaponType,
    pub ascension: Vec<Ascension>,
    pub stats: CharacterStats,
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
        let url = format!("http://localhost:3000/api/characters/{}", character);
        let url = Url::parse(&*url).expect("Can't convert url");
        return reqwest::get(url).await.expect("Can't access Url").json::<Character>().await.expect("Wrong json format");
    }

    #[allow(dead_code)]
    async fn get_all() -> Vec<Box<str>> {
        let url = format!("http://localhost:3000/api/characters");
        let url = Url::parse(&*url).expect("Can't convert url");
        return reqwest::get(url).await.expect("Can't access Url").json::<Vec<Box<str>>>().await.expect("Wrong json format");
    }

    #[allow(dead_code)]
    pub(crate) async fn search(character: &str) -> Vec<Character> {
        let url = format!("http://localhost:3000/api/characters/search/{}", character);
        let url = Url::parse(&*url).expect("Can't convert url");
        return reqwest::get(url).await.expect("Can't access Url").json::<Vec<Character>>().await.expect("Wrong json format");
    }
}

#[allow(dead_code)]
pub async fn test_character () {
    for a in Character::get_all().await {
        println!("{}", a);
        let char = Character::get(&a).await;
        println!("Name : {}", char.name);
    }
}