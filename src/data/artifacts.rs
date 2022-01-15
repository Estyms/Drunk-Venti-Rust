use reqwest::Url;
use serde_derive::{Serialize, Deserialize};
use serenity::builder::CreateEmbed;
use crate::data::domains::Domain;

#[derive(Serialize, Deserialize, Debug)]
pub struct Set {
    pub goblet: Option<Box<str>>,
    pub plume: Option<Box<str>>,
    pub circlet: Option<Box<str>>,
    pub flower: Option<Box<str>>,
    pub sands: Option<Box<str>>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Artifact {
    pub id: Box<str>,
    pub name: Box<str>,
    pub set_piece: Vec<u32>,
    pub sets: Set,
    pub bonuses: Vec<Box<str>>,
    pub rarity: Vec<u8>,
    pub domain: Option<Box<str>>
}

impl Artifact{
    pub async fn get(artifact: &str) -> Artifact {
        let url = format!("http://localhost:3000/api/artifacts/{}", artifact);
        let url = Url::parse(&*url).expect("Can't convert url");
        return reqwest::get(url).await.expect("Can't access Url").json::<Artifact>().await.expect("Wrong json format");
    }

    async fn get_all() -> Vec<Box<str>> {
        let url = format!("http://localhost:3000/api/artifacts");
        let url = Url::parse(&*url).expect("Can't convert url");
        return reqwest::get(url).await.expect("Can't access Url").json::<Vec<Box<str>>>().await.expect("Wrong json format");
    }

    #[allow(dead_code)]
    pub(crate) async fn search(artifact: &str) -> Vec<Artifact> {
        let url = format!("http://localhost:3000/api/artifacts/search/{}", artifact);
        let url = Url::parse(&*url).expect("Can't convert url");
        return reqwest::get(url).await.expect("Can't access Url").json::<Vec<Artifact>>().await.expect("Wrong json format");
    }

    #[allow(dead_code)]
    pub async fn to_embed(&self) -> CreateEmbed {
        let mut embed = CreateEmbed::default();

        embed.title(format!("{} | {}", self.name, ":star:".repeat(self.rarity.get(0).expect("").to_owned() as usize)));

        embed.thumbnail(format!("https://raw.githubusercontent.com/MadeBaruna/paimon-moe/main/static/images/artifacts/{}_circlet.png", self.id));

        for i in 0..self.set_piece.len(){
            embed.field(format!("{}-Pieces", self.set_piece.get(i).expect("No such piece set")),
                        format!("{}", self.bonuses.get(i).expect("Mo such effects")),
                        false
            );
        }

        match &self.domain {
            Some(d) => {
                let domain = Domain::get(d).await;
                embed.field("Domain", domain.name(), true);
            }
            _ => {}
        }

        return embed;
    }

    #[allow(dead_code)]
    pub fn id(&self) -> &str {
        return self.id.as_ref()
    }

    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        return self.name.as_ref()
    }
}

pub async fn test_artifacts() {
    for a in Artifact::get_all().await {
        println!("{}", a);
        let arti = Artifact::get(&a).await;
        println!("Name : {}\nCirclet : {}\nBonuses {:?}", arti.name, arti.sets.goblet.unwrap_or(Box::from("")), arti.bonuses);
    }

}
