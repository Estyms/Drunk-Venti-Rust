use std::env;
use mongodb::bson::{Bson, doc, Document, from_bson, to_bson};
use mongodb::{Client, Collection, options::ClientOptions};
use futures::stream::{TryStreamExt};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StatusMessage {
    pub(crate) guild_id: i64,
    pub(crate) message_id: i64,
    pub(crate) channel_id: i64
}

#[allow(dead_code)]
pub async fn add_discord_status_message(status_message: StatusMessage) -> bool {
    let client = get_mongo_client().await;

    let serialized = to_bson(&status_message).expect("Can't serialize status_message");
    let document = serialized.as_document().unwrap();

    let collection: Collection<Document> = client.database("drunk_venti").collection("StatusMessages");
    let updated = collection.update_one(doc! {"guild_id": status_message.guild_id}, doc! {"$set" : document}, None).await;
    let is_ok = &updated.is_ok();
    match updated.unwrap() {
        e if e.matched_count == 0 => {
            let inserted = collection.insert_one(document, None).await;
            inserted.is_ok()
        }
        _ => *is_ok
    }
}

#[allow(dead_code)]
pub async fn get_discord_status_message(gid: &u64) -> Option<StatusMessage> {
    let client = get_mongo_client().await;

    let status_messages: Collection<Document> = client.database("drunk_venti").collection("StatusMessages");

    let infos = status_messages.find_one(doc! {"guild_id" : *gid as i64}, None).await.expect("Can't find one");


    match infos {
        Some(i) => {
            let m_infos: StatusMessage = from_bson(Bson::Document(i)).expect("Can't get");
            Some(m_infos)
        }
        _ => None
    }
}

#[allow(dead_code)]
pub async fn get_all_status_messages() -> Vec<StatusMessage> {
    let client = get_mongo_client().await;

    let collection: Collection<StatusMessage> = client.database("drunk_venti").collection::<StatusMessage>("StatusMessages");
    let documents = collection.find(None, None).await.expect("Can't get everything");
    let all_docs: Vec<StatusMessage> = documents.try_collect().await.unwrap_or_else(|_| vec![]);
    all_docs
}

#[allow(dead_code)]
async fn get_mongo_client() -> Client {
    let host = env::var("MONGO_HOST").unwrap();
    let port = env::var("MONGO_PORT").unwrap();
    let mut client_options = ClientOptions::parse(format!("mongodb://{}:{}/?readPreference=primary&appname=MongoDB%20Compass&directConnection=true&ssl=false", host, port)).await.expect("Can't connect to db");
    client_options.app_name = Some("Drunk Venti".to_string());
    client_options.default_database = Some("drunk_venti".to_string());

    Client::with_options(client_options).expect("Can't add options")
}