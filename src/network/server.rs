use crate::ws::YouTube;
use axum::{extract::State, http::StatusCode, routing::patch, Json, Router};
use std::net::SocketAddr;
use tokio::sync::mpsc::Sender;

pub struct Server {
    router: Router,
    serve_from: SocketAddr,
}

impl Server {
    pub fn new(tx_state: Sender<YouTube>, serve_from: SocketAddr) -> Server {
        let router = Router::new()
            .route("/services/youtube", patch(play_youtube_video))
            .with_state(tx_state);

        Server { router, serve_from }
    }

    pub async fn bind(&self) {
        axum::Server::bind(&self.serve_from)
            .serve(self.router.clone().into_make_service())
            .await
            .unwrap();
    }
}

async fn play_youtube_video(
    State(tx): State<Sender<YouTube>>,
    Json(payload): Json<YouTube>,
) -> (StatusCode, Json<u8>) {
    match tx.send(payload).await {
        Ok(_) => (StatusCode::OK, Json(0)),
        Err(err) => {
            println!("Receiver send error: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(1))
        }
    }
}
