use std::env;
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
    pub start_timestamp: Option<u64>,
    pub end_timestamp: Option<u64>,
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
        let host = env::var("API_HOST").unwrap();
        let port = env::var("API_PORT").unwrap();
        let url = format!("http://{}:{}/api/events/current", host, port);
        let url = Url::parse(&url).expect("Can't convert url");
        reqwest::get(url).await.expect("Can't access Url").json::<Vec<Event>>().await.expect("Wrong json format")
    }

    #[allow(dead_code)]
    pub(crate) async fn get_upcoming() -> Vec<Event> {
        let host = env::var("API_HOST").unwrap();
        let port = env::var("API_PORT").unwrap();
        let url = format!("http://{}:{}/api/events/upcoming", host, port);
        let url = Url::parse(&url).expect("Can't convert url");
        reqwest::get(url).await.expect("Can't access Url").json::<Vec<Event>>().await.expect("Wrong json format")
    }

    #[allow(dead_code)]
    async fn get_all() -> Vec<Event> {
        let host = env::var("API_HOST").unwrap();
        let port = env::var("API_PORT").unwrap();
        let url = format!("http://{}:{}/api/events", host, port);
        let url = Url::parse(&url).expect("Can't convert url");
        reqwest::get(url).await.expect("Can't access Url").json::<Vec<Event>>().await.expect("Wrong json format")
    }
}

#[test]
fn test_events() {
    let data = std::fs::read_to_string("test/events.json").expect("No events test file");
    serde_json::from_str::<Vec<Event>>(&data).expect("Didn't work");
}