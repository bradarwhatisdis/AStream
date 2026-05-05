mod signaling;
mod audio;
mod webrtc_handler;
mod discovery;

use signaling::SignalingState;
use discovery::DeviceInfo;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("AStream PC v0.1.0 - Starting...");

    let _webrtc_manager = webrtc_handler::WebRTCManager::new();
    let signaling_state = Arc::new(SignalingState::new());

    println!("Starting signaling server...");
    let state_clone = signaling_state.clone();
    tokio::spawn(async move {
        if let Err(e) = signaling::start_signaling_server("0.0.0.0:8080", state_clone).await {
            eprintln!("Signaling server error: {}", e);
        }
    });

    println!("Starting device discovery...");
    let device_info = DeviceInfo::default();
    tokio::spawn(async move {
        if let Err(e) = discovery::start_discovery_server(device_info).await {
            eprintln!("Discovery server error: {}", e);
        }
    });

    println!("AStream PC ready.");
    println!("Signaling: ws://0.0.0.0:8080");
    println!("Waiting for connections...");

    loop {
        sleep(Duration::from_secs(1)).await;
    }
}
