use serde_derive::{Serialize, Deserialize};
use crate::data::items::Item;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WeaponType {
    pub id: Box<str>,
    pub name: Box<str>
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AscensionItem {
    pub item: Item,
    pub amount: u32
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Ascension {
    pub items: Vec<AscensionItem>,
    pub mora: u64
}