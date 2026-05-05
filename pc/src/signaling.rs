use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tokio_tungstenite::{accept_async, tungstenite::Message};
use futures_util::stream::StreamExt;

#[derive(Debug, Serialize, Deserialize)]
pub enum SignalMessage {
    Offer(String),
    Answer(String),
    IceCandidate(String),
    Heartbeat,
}

pub async fn start_signaling_server(addr: &str) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(addr).await?;
    println!("Signaling server listening on {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(async move {
            if let Ok(ws) = accept_async(stream).await {
                println!("New WebSocket connection");
                handle_connection(ws).await;
            }
        });
    }
    Ok(())
}

async fn handle_connection(ws: tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>) {
    let (_, mut read) = ws.split();
    while let Some(Ok(msg)) = read.next().await {
        if let Message::Text(text) = msg {
            if let Ok(signal) = serde_json::from_str::<SignalMessage>(&text) {
                match signal {
                    SignalMessage::Heartbeat => println!("Heartbeat received"),
                    _ => println!("Signal received: {:?}", signal),
                }
            }
        }
    }
}
