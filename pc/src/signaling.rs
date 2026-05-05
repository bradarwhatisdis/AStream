use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tokio_tungstenite::{accept_async, tungstenite::Message};
use futures_util::stream::StreamExt;
use futures_util::sink::SinkExt;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SignalMessage {
    Offer { sdp: String },
    Answer { sdp: String },
    IceCandidate { candidate: String, sdp_mid: String, sdp_mline_index: u16 },
    Heartbeat,
    RequestOffer,
}

pub type SignalSender = tokio::sync::mpsc::UnboundedSender<Message>;
pub type SignalReceiver = tokio::sync::mpsc::UnboundedReceiver<Message>;

pub struct SignalingState {
    pub peers: Arc<Mutex<HashMap<String, SignalSender>>>,
}

impl SignalingState {
    pub fn new() -> Self {
        Self {
            peers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn add_peer(&self, id: String, sender: SignalSender) {
        self.peers.lock().unwrap().insert(id, sender);
    }

    pub fn remove_peer(&self, id: &str) {
        self.peers.lock().unwrap().remove(id);
    }

    pub fn broadcast(&self, message: SignalMessage) {
        let msg = serde_json::to_string(&message).unwrap();
        let peers = self.peers.lock().unwrap();
        for (id, sender) in peers.iter() {
            if let Err(e) = sender.send(Message::Text(msg.clone())) {
                eprintln!("Failed to send to {}: {}", id, e);
            }
        }
    }
}

pub async fn start_signaling_server(addr: &str, state: Arc<SignalingState>) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(addr).await?;
    println!("Signaling server listening on {}", addr);

    let mut peer_counter = 0;

    while let Ok((stream, _)) = listener.accept().await {
        peer_counter += 1;
        let peer_id = format!("peer-{}", peer_counter);
        let state_clone = state.clone();

        tokio::spawn(async move {
            if let Ok(ws) = accept_async(stream).await {
                println!("New peer connected: {}", peer_id);

                let (mut write, mut read) = ws.split();
                let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<Message>();

                state_clone.add_peer(peer_id.clone(), tx);

                // Forward messages from channel to WebSocket
                let peer_id_clone = peer_id.clone();
                let write_task = tokio::spawn(async move {
                    while let Some(msg) = rx.recv().await {
                        if write.send(msg).await.is_err() {
                            break;
                        }
                    }
                    println!("Write task ended for {}", peer_id_clone);
                });

                // Read messages from WebSocket
                let state_for_read = state_clone.clone();
                let peer_id_for_read = peer_id.clone();
                let read_task = tokio::spawn(async move {
                    while let Some(Ok(msg)) = read.next().await {
                        if let Message::Text(text) = msg {
                            if let Ok(signal) = serde_json::from_str::<SignalMessage>(&text) {
                                match signal {
                                    SignalMessage::Heartbeat => {},
                                    _ => {
                                        println!("Signal from {}: {:?}", peer_id_for_read, signal);
                                        state_for_read.broadcast(signal);
                                    }
                                }
                            }
                        }
                    }
                    println!("Read task ended for {}", peer_id_for_read);
                });

                // Wait for either task to finish
                tokio::select! {
                    _ = write_task => {},
                    _ = read_task => {},
                }

                state_inner.remove_peer(&peer_id);
                println!("Peer disconnected: {}", peer_id);
            }
        });
    }
    Ok(())
}
