use std::env;
use reqwest::Url;
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DomainMonster {
    pub id: Box<str>,
    pub name: Box<str>,
    pub count: u8,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DomainReward {
    pub adventure_exp: Box<str>,
    pub mora: Box<str>,
    pub friendship_exp: Box<str>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DomainDifficulty {
    pub s: u64,
    pub id: Box<str>,
    pub name: Box<str>,
    pub ar: u8,
    pub level: u8,
    pub reward: DomainReward,
    pub monsters: Vec<DomainMonster>,
    pub disorder: Vec<Box<str>>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Domain {
    pub name: Box<str>,
    pub domains: Vec<DomainDifficulty>,
    pub artifacts: Vec<Box<str>>,
}

impl Domain {
    #[allow(dead_code)]
    pub async fn get(domain: &str) -> Domain {
        let host = env::var("API_HOST").unwrap();
        let port = env::var("API_PORT").unwrap();
        let url = format!("http://{}:{}/api/domains/{}", host, port, domain);
        let url = Url::parse(&url).expect("Can't convert url");
        reqwest::get(url).await.expect("Can't access Url").json::<Domain>().await.expect("Wrong json format")
    }

    #[allow(dead_code)]
    async fn get_all() -> Vec<Box<str>> {
        let host = env::var("API_HOST").unwrap();
        let port = env::var("API_PORT").unwrap();
        let url = format!("http://{}:{}/api/domains", host, port);
        let url = Url::parse(&url).expect("Can't convert url");
        reqwest::get(url).await.expect("Can't access Url").json::<Vec<Box<str>>>().await.expect("Wrong json format")
    }

    #[allow(dead_code)]
    pub fn name(&self) -> String {
        self.name.to_string()
    }
}


#[test]
fn test_items() {
    let data = std::fs::read_to_string("test/domain.json").expect("No Domain test file");
    serde_json::from_str::<Domain>(&data).expect("Didn't work");
}