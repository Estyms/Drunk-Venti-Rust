use reqwest::Url;
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: Box<str>,
    pub name: Box<str>,
    pub day: Option<Vec<Box<str>>>,
    pub rarity: Option<u8>,
    pub parent: Option<Box<str>>
}

impl Item{
    async fn get(item: &str) -> Item {
        let url = format!("http://localhost:3000/api/items/{}", item);
        let url = Url::parse(&*url).expect("Can't convert url");
        return reqwest::get(url).await.expect("Can't access Url").json::<Item>().await.expect("Wrong json format");
    }

    async fn get_all() -> Vec<Box<str>> {
        let url = format!("http://localhost:3000/api/items");
        let url = Url::parse(&*url).expect("Can't convert url");
        return reqwest::get(url).await.expect("Can't access Url").json::<Vec<Box<str>>>().await.expect("Wrong json format");
    }

    #[allow(dead_code)]
    async fn search(item: &str) -> Vec<Item> {
        let url = format!("http://localhost:3000/api/items/search/{}", item);
        let url = Url::parse(&*url).expect("Can't convert url");
        return reqwest::get(url).await.expect("Can't access Url").json::<Vec<Item>>().await.expect("Wrong json format");
    }
}

pub async fn test_items() {
    println!("List all items : {:?}", Item::get_all().await);

    let item = Item::get("relic_from_guyun").await;
    println!("Name : {}\nDays : {:?}", item.name, item.day.unwrap_or(vec![]));

    let item = Item::get("mora").await;
    println!("Name : {}\nDays : {:?}", item.name, item.day.unwrap_or(vec![]));
}