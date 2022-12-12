use std::env;
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

pub fn get_real_artifact_name(artifact: &str) -> &str {
    match artifact {
        "+18%_atk_set" => "gladiators_finale",
        "+20%_energy_recharge" => "emblem_of_severed_fate",
        "+25%_physical_dmg" => "bloodstained_chivalry",
        "+80_em" => "gilded_dreams",
        "+15%_healing_bonus_set" => "ocean-hued_clam",
        _ => artifact
    }
}

impl Artifact{


    #[allow(dead_code)]
    pub async fn get(artifact: &str) -> Artifact {
        let artifact = get_real_artifact_name(artifact);
        let host = env::var("API_HOST").unwrap();
        let port = env::var("API_PORT").unwrap();
        let url = format!("http://{}:{}/api/artifacts/{}", host, port, artifact);
        let url = Url::parse(&*url).expect("Can't convert url");
        return reqwest::get(url).await.expect("Can't access Url").json::<Artifact>().await.expect("Wrong json format");
    }

    #[allow(dead_code)]
    async fn get_all() -> Vec<Box<str>> {
        let host = env::var("API_HOST").unwrap();
        let port = env::var("API_PORT").unwrap();
        let url = format!("http://{}:{}/api/artifacts", host, port);
        let url = Url::parse(&*url).expect("Can't convert url");
        return reqwest::get(url).await.expect("Can't access Url").json::<Vec<Box<str>>>().await.expect("Wrong json format");
    }

    #[allow(dead_code)]
    pub(crate) async fn search(artifact: &str) -> Vec<Artifact> {
        let artifact = get_real_artifact_name(artifact);
        let host = env::var("API_HOST").unwrap();
        let port = env::var("API_PORT").unwrap();
        let url = format!("http://{}:{}/api/artifacts/search/{}", host, port, artifact);
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

#[test]
fn test_artifact() {
    let data = std::fs::read_to_string("test/artifact.json").expect("No Artifact test file");
    serde_json::from_str::<Artifact>(&data).expect("Didn't work");
}

#[test]
fn test_real_artfifact_name() {
    match get_real_artifact_name("+18%_atk_set") {
        "gladiators_finale" => {}
        _ => panic!()
    };

    match get_real_artifact_name("lasfhkalkfhafsk") {
        "lasfhkalkfhafsk" => {}
        _ => panic!()
    };

}