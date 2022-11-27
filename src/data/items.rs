use std::env;
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
    #[allow(dead_code)]
    async fn get(item: &str) -> Item {
        let host = env::var("API_HOST").unwrap();
        let port = env::var("API_PORT").unwrap();
        let url = format!("http://{}:{}/api/items/{}", host, port, item);
        let url = Url::parse(&*url).expect("Can't convert url");
        return reqwest::get(url).await.expect("Can't access Url").json::<Item>().await.expect("Wrong json format");
    }
    #[allow(dead_code)]
    async fn get_all() -> Vec<Box<str>> {
        let host = env::var("API_HOST").unwrap();
        let port = env::var("API_PORT").unwrap();
        let url = format!("http://{}:{}/api/items", host, port);
        let url = Url::parse(&*url).expect("Can't convert url");
        return reqwest::get(url).await.expect("Can't access Url").json::<Vec<Box<str>>>().await.expect("Wrong json format");
    }

    #[allow(dead_code)]
    async fn search(item: &str) -> Vec<Item> {
        let host = env::var("API_HOST").unwrap();
        let port = env::var("API_PORT").unwrap();
        let url = format!("http://{}:{}/api/items/search/{}", host, port, item);
        let url = Url::parse(&*url).expect("Can't convert url");
        return reqwest::get(url).await.expect("Can't access Url").json::<Vec<Item>>().await.expect("Wrong json format");
    }
}

#[test]
fn test_item() {
    let data = std::fs::read_to_string("test/item.json").expect("No item test file");
    serde_json::from_str::<Item>(&data).expect("Didn't work");
}