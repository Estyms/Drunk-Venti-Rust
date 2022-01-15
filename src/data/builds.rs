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
    pub sands: Box<str>,
    pub goblet: Box<str>,
    pub circlet: Box<str>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Role{
    pub name: Box<str>,
    pub recommended: bool,
    pub weapons: Vec<RoleWeapon>,
    pub artifacts: Vec<Vec<Box<str>>>,
    pub main_stats: RoleStat,
    pub sub_stats: Vec<Box<str>>,
    pub talent: Vec<Box<str>>,
    pub tip: Box<str>,
    pub note: Box<str>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Builds {
    pub roles: Vec<Role>
}

impl Builds {
    pub(crate) async fn get(build: &str) -> Builds {
        let url = format!("http://localhost:3000/api/builds/{}", build);
        let url = Url::parse(&*url).expect("Can't convert url");
        return reqwest::get(url).await.expect("Can't access Url").json::<Builds>().await.expect("Wrong json format");
    }

    async fn get_all() -> Vec<Box<str>> {
        let url = format!("http://localhost:3000/api/builds");
        let url = Url::parse(&*url).expect("Can't convert url");
        return reqwest::get(url).await.expect("Can't access Url").json::<Vec<Box<str>>>().await.expect("Wrong json format");
    }
}


pub async fn test_builds() {
    for a in Builds::get_all().await {
        println!("------------------");
        let builds = Builds::get(&a).await;
        println!("Roles for {} : {:?}\n", a, builds.roles.into_iter().map(|x| x.name).collect::<Vec<Box<str>>>());
    }

}