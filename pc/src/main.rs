mod signaling;
mod audio;
mod webrtc_handler;

use std::sync::Arc;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("AStream PC v0.1.0 - Starting...");

    let audio_config = audio::AudioConfig::default();
    println!("Audio config: {:?}", audio_config);

    let input_device = audio::get_default_input_device();
    let output_device = audio::get_default_output_device();

    println!("Input: {:?}", input_device);
    println!("Output: {:?}", output_device);

    let signaling_state = Arc::new(signaling::SignalingState::new());

    println!("Starting signaling server...");
    let state_clone = signaling_state.clone();
    tokio::spawn(async move {
        if let Err(e) = signaling::start_signaling_server("0.0.0.0:8080", state_clone).await {
            eprintln!("Signaling server error: {}", e);
        }
    });

    println!("AStream PC ready. Signaling on ws://0.0.0.0:8080");
    println!("Waiting for connections...");

    loop {
        sleep(Duration::from_secs(1)).await;
    }
}
