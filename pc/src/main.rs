mod signaling;
mod audio;
mod webrtc_handler;

use signaling::SignalingState;
use webrtc_handler::WebRTCManager;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("AStream PC v0.1.0 - Starting...");

    let audio_config = audio::AudioConfig::default();
    println!("Audio config: {:?}", audio_config);

    let _webrtc_manager = WebRTCManager::new();
    let signaling_state = Arc::new(SignalingState::new());

    println!("Starting signaling server...");
    let state_clone = signaling_state.clone();
    tokio::spawn(async move {
        if let Err(e) = signaling::start_signaling_server("0.0.0.0:8080", state_clone).await {
            eprintln!("Signaling server error: {}", e);
        }
    });

    println!("AStream PC ready. Signaling on ws://0.0.0.0:8080");
    println!("Waiting for connections...");

    // Main loop - handle signaling messages and WebRTC
    loop {
        sleep(Duration::from_secs(1)).await;
        // TODO: Process signaling messages and update WebRTC state
    }
}
