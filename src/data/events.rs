use reqwest::Url;
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub name: Box<str>,
    pub image: Option<Box<str>>,
    pub start: Box<str>,
    pub end: Box<str>,
    pub url: Option<Box<str>>,
    pub start_timestamp: u64,
    pub end_timestamp: u64,
    pub show_on_home: Option<bool>
}

impl Clone for Event {
    fn clone(&self) -> Self {
        let x = serde_json::to_string(&self).expect("");
        serde_json::from_str::<Event>(x.as_str()).expect("")
    }
}



impl Event {
    #[allow(dead_code)]
    pub(crate) async fn get_current() -> Vec<Event> {
        let url = format!("http://localhost:3000/api/events/current");
        let url = Url::parse(&*url).expect("Can't convert url");
        return reqwest::get(url).await.expect("Can't access Url").json::<Vec<Event>>().await.expect("Wrong json format");
    }

    #[allow(dead_code)]
    pub(crate) async fn get_upcoming() -> Vec<Event> {
        let url = format!("http://localhost:3000/api/events/upcoming");
        let url = Url::parse(&*url).expect("Can't convert url");
        return reqwest::get(url).await.expect("Can't access Url").json::<Vec<Event>>().await.expect("Wrong json format");
    }

    #[allow(dead_code)]
    async fn get_all() -> Vec<Event> {
        let url = format!("http://localhost:3000/api/events");
        let url = Url::parse(&*url).expect("Can't convert url");
        return reqwest::get(url).await.expect("Can't access Url").json::<Vec<Event>>().await.expect("Wrong json format");
    }
}

#[allow(dead_code)]
pub async fn test_events() {
    let upcoming = Event::get_upcoming().await;
    println!("UPCOMING EVENTS\n--------");
    for event in upcoming {
        println!("Name : {}", event.name);
    }

    let current = Event::get_current().await;
    println!("CURRENT EVENTS\n--------");
    for event in current {
        println!("Name : {}", event.name);
    }
}