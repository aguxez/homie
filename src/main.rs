mod network;

use futures::{SinkExt, StreamExt};
use network::{server::Server, ws};
use std::net::SocketAddr;
use tokio::sync::mpsc::channel;
use tokio_tungstenite::tungstenite::Message;

#[tokio::main]
async fn main() {
    let ws_stream = ws::connect_async().await;

    let (mut ws_tx, mut ws_rx) = ws_stream.split();

    ws_tx.send(Message::Text(HELLO.into())).await.unwrap();

    let (tx, mut rx) = channel::<ws::YouTube>(1);
    let serve_from = SocketAddr::from(([127, 0, 0, 1], 9090));
    let server = Server::new(tx, serve_from);

    tokio::spawn(async move {
        loop {
            if let Some(chan_msg) = rx.recv().await {
                ws::open_yt(&mut ws_tx, &chan_msg).await;
            }
        }
    });

    tokio::spawn(async move {
        ws::wait_msg(&mut ws_rx).await;
    });

    server.bind().await;
}

static HELLO: &str = r##"
{"id":"register_0","payload":{"client-key":"36aba158f01f5c09ce3eb658a61c65cc","forcePairing":false,"manifest":{"appVersion":"1.1","manifestVersion":1,"permissions":["LAUNCH","LAUNCH_WEBAPP","APP_TO_APP","CLOSE","TEST_OPEN","TEST_PROTECTED","CONTROL_AUDIO","CONTROL_DISPLAY","CONTROL_INPUT_JOYSTICK","CONTROL_INPUT_MEDIA_RECORDING","CONTROL_INPUT_MEDIA_PLAYBACK","CONTROL_INPUT_TV","CONTROL_POWER","READ_APP_STATUS","READ_CURRENT_CHANNEL","READ_INPUT_DEVICE_LIST","READ_NETWORK_STATE","READ_RUNNING_APPS","READ_TV_CHANNEL_LIST","WRITE_NOTIFICATION_TOAST","READ_POWER_STATE","READ_COUNTRY_INFO","READ_SETTINGS","CONTROL_TV_SCREEN","CONTROL_TV_STANBY","CONTROL_FAVORITE_GROUP","CONTROL_USER_INFO","CHECK_BLUETOOTH_DEVICE","CONTROL_BLUETOOTH","CONTROL_TIMER_INFO","STB_INTERNAL_CONNECTION","CONTROL_RECORDING","READ_RECORDING_STATE","WRITE_RECORDING_LIST","READ_RECORDING_LIST","READ_RECORDING_SCHEDULE","WRITE_RECORDING_SCHEDULE","READ_STORAGE_DEVICE_LIST","READ_TV_PROGRAM_INFO","CONTROL_BOX_CHANNEL","READ_TV_ACR_AUTH_TOKEN","READ_TV_CONTENT_STATE","READ_TV_CURRENT_TIME","ADD_LAUNCHER_CHANNEL","SET_CHANNEL_SKIP","RELEASE_CHANNEL_SKIP","CONTROL_CHANNEL_BLOCK","DELETE_SELECT_CHANNEL","CONTROL_CHANNEL_GROUP","SCAN_TV_CHANNELS","CONTROL_TV_POWER","CONTROL_WOL"],"signatures":[{"signature":"eyJhbGdvcml0aG0iOiJSU0EtU0hBMjU2Iiwia2V5SWQiOiJ0ZXN0LXNpZ25pbmctY2VydCIsInNpZ25hdHVyZVZlcnNpb24iOjF9.hrVRgjCwXVvE2OOSpDZ58hR+59aFNwYDyjQgKk3auukd7pcegmE2CzPCa0bJ0ZsRAcKkCTJrWo5iDzNhMBWRyaMOv5zWSrthlf7G128qvIlpMT0YNY+n/FaOHE73uLrS/g7swl3/qH/BGFG2Hu4RlL48eb3lLKqTt2xKHdCs6Cd4RMfJPYnzgvI4BNrFUKsjkcu+WD4OO2A27Pq1n50cMchmcaXadJhGrOqH5YmHdOCj5NSHzJYrsW0HPlpuAx/ECMeIZYDh6RMqaFM2DXzdKX9NmmyqzJ3o/0lkk/N97gfVRLW5hA29yeAwaCViZNCP8iC9aO0q9fQojoa7NQnAtw==","signatureVersion":1}],"signed":{"appId":"com.lge.test","created":"20140509","localizedAppNames":{"":"LG Remote App","ko-KR":"????????? ???","zxx-XX":"???? R????ot?? A????"},"localizedVendorNames":{"":"LG Electronics"},"permissions":["TEST_SECURE","CONTROL_INPUT_TEXT","CONTROL_MOUSE_AND_KEYBOARD","READ_INSTALLED_APPS","READ_LGE_SDX","READ_NOTIFICATIONS","SEARCH","WRITE_SETTINGS","WRITE_NOTIFICATION_ALERT","CONTROL_POWER","READ_CURRENT_CHANNEL","READ_RUNNING_APPS","READ_UPDATE_INFO","UPDATE_FROM_REMOTE_APP","READ_LGE_TV_INPUT_EVENTS","READ_TV_CURRENT_TIME"],"serial":"2f930e2d2cfe083771f68e4fe7bb07","vendorId":"com.lge"}},"pairingType":"PROMPT"},"type":"register"}
"##;
