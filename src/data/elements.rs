use std::env;
use reqwest::Url;
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Element {
    pub id: Box<str>,
    pub name: Box<str>,
    pub simple_name: Box<str>,
    pub color: u64,
}

impl Element {
    #[allow(dead_code)]
    async fn get(element: &str) -> Element {
        let host = env::var("API_HOST").unwrap();
        let port = env::var("API_PORT").unwrap();
        let url = format!("http://{}:{}/api/elements/{}", host, port, element);
        let url = Url::parse(&url).expect("Can't convert url");
        reqwest::get(url).await.expect("Can't access Url").json::<Element>().await.expect("Wrong json format")
    }

    #[allow(dead_code)]
    async fn get_all() -> Vec<Box<str>> {
        let host = env::var("API_HOST").unwrap();
        let port = env::var("API_PORT").unwrap();
        let url = format!("http://{}:{}/api/elements", host, port);
        let url = Url::parse(&url).expect("Can't convert url");
        reqwest::get(url).await.expect("Can't access Url").json::<Vec<Box<str>>>().await.expect("Wrong json format")
    }
}

#[test]
fn test_element() {
    let data = std::fs::read_to_string("test/element.json").expect("No Element test file");
    serde_json::from_str::<Element>(&data).expect("Didn't work");
}