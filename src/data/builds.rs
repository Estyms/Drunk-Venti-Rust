use std::env;
use reqwest::Url;
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RoleWeapon {
    pub id: Box<str>,
    pub refine: Option<Vec<u8>>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RoleStat {
    pub sands: Vec<Box<str>>,
    pub goblet: Vec<Box<str>>,
    pub circlet: Vec<Box<str>>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Role{
    pub recommended: bool,
    pub weapons: Vec<RoleWeapon>,
    pub artifacts: Vec<Vec<Box<str>>>,
    pub main_stats: RoleStat,
    pub sub_stats: Vec<Box<str>>,
    pub talent: Vec<Box<str>>,
    pub tip: Option<String>,
    pub note: Box<str>,
    pub name: Box<str>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Builds {
    pub roles: Vec<Role>
}

impl Builds {
    #[allow(dead_code)]
    pub(crate) async fn get(build: &str) -> Builds {
        let host = env::var("API_HOST").unwrap();
        let port = env::var("API_PORT").unwrap();
        let url = format!("http://{}:{}/api/builds/{}", host, port, build);
        let url = Url::parse(&url).expect("Can't convert url");
        reqwest::get(url).await.expect("Can't access Url").json::<Builds>().await.expect("Wrong json format")
    }

    #[allow(dead_code)]
    async fn get_all() -> Vec<Box<str>> {
        let host = env::var("API_HOST").unwrap();
        let port = env::var("API_PORT").unwrap();
        let url = format!("http://{}:{}/api/builds", host, port);
        let url = Url::parse(&url).expect("Can't convert url");
        reqwest::get(url).await.expect("Can't access Url").json::<Vec<Box<str>>>().await.expect("Wrong json format")
    }
}

#[test]
fn test_build() {
    let data = std::fs::read_to_string("test/build.json").expect("No build test file");
    serde_json::from_str::<Builds>(&data).expect("Didn't work");
}