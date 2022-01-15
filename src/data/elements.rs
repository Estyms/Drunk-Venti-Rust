use reqwest::Url;
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Element {
    pub id: Box<str>,
    pub name: Box<str>,
    pub simple_name: Box<str>,
    pub color: u64
}

impl Element{
    async fn get(element: &str) -> Element {
        let url = format!("http://localhost:3000/api/elements/{}", element);
        let url = Url::parse(&*url).expect("Can't convert url");
        return reqwest::get(url).await.expect("Can't access Url").json::<Element>().await.expect("Wrong json format");
    }

    async fn get_all() -> Vec<Box<str>> {
        let url = format!("http://localhost:3000/api/elements");
        let url = Url::parse(&*url).expect("Can't convert url");
        return reqwest::get(url).await.expect("Can't access Url").json::<Vec<Box<str>>>().await.expect("Wrong json format");
    }
}

pub async fn test_elem() {
    println!("All elems : {:?}", Element::get_all().await);

    let geo = Element::get("geo").await;
    println!("Name : {}\nColor : #{:x}", geo.name, geo.color);
}