use crate::db::conn::Pool;
use crate::key;

use futures::stream::{SplitSink, SplitStream};
use futures::{SinkExt, StreamExt};
use serde::Deserialize;
use serde_json::{json, Value};
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};
use url::Url;

const WS_SERVER: &str = "ws://192.168.1.6:3000";

#[derive(Deserialize, Debug)]
struct Response {
    #[serde(rename = "type")]
    command_type: String,
    #[serde(rename = "id")]
    command_id: String,
    payload: Value,
}

#[derive(Deserialize, Debug)]
pub struct YouTube {
    video_id: String,
}

#[derive(Deserialize)]
struct Pairing {
    #[serde(rename = "client-key")]
    key: String,
}

pub async fn connect_async() -> WebSocketStream<MaybeTlsStream<TcpStream>> {
    let (ws_stream, _) = tokio_tungstenite::connect_async(Url::parse(WS_SERVER).unwrap())
        .await
        .expect("Cannot connect");
    ws_stream
}

pub async fn wait_msg(
    ws_rx: &mut SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    pool: &mut Pool,
) {
    loop {
        if let Some(Ok(raw_msg)) = ws_rx.next().await {
            if let Ok(msg) = serde_json::from_str::<Response>(raw_msg.to_text().unwrap()) {
                handle_socket_msg(msg, pool);
            }
        }
    }
}

fn handle_socket_msg(msg: Response, pool: &mut Pool) {
    match (msg.command_type.as_str(), msg.command_id.as_str()) {
        ("response", "register_0") => println!("Please accept pairing request in your TV"),
        ("registered", "register_0") => {
            let pairing: Pairing = serde_json::from_value(msg.payload).unwrap();
            key::insert_key(pairing.key, pool).unwrap();
        }
        _ => println!("Unhandled message: {:?}", msg),
    }
}

pub async fn open_yt(
    ws_tx: &mut SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    payload: &YouTube,
) {
    let link = format!(
        "https://youtube.com/watch?v={video_id}",
        video_id = payload.video_id
    );

    let payload: Value = json!({
        "type": "request",
        "id": "youtube_1",
        "uri": "ssap://system.launcher/launch",
        "payload": {
            "id": "youtube.leanback.v4",
            "params": { "contentTarget": link }
        }
    });

    ws_tx
        .send(Message::Text(payload.to_string()))
        .await
        .unwrap();
}
