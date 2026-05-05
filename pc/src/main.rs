mod signaling;
mod audio;
mod webrtc_handler;

use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("AStream PC v0.1.0 - Starting...");

    let audio_config = audio::AudioConfig::default();
    println!("Audio config: {:?}Hz, {} channels, buffer: {} samples", 
             audio_config.sample_rate, audio_config.channels, audio_config.buffer_size);

    let input_device = audio::get_default_input_device();
    let output_device = audio::get_default_output_device();

    match (&input_device, &output_device) {
        (Some(in_dev), Some(out_dev)) => {
            println!("Input: {}", in_dev.name().unwrap_or_default());
            println!("Output: {}", out_dev.name().unwrap_or_default());
        }
        _ => println!("Warning: Could not find default audio devices"),
    }

    println!("Starting signaling server...");
    tokio::spawn(async {
        if let Err(e) = signaling::start_signaling_server("0.0.0.0:8080").await {
            eprintln!("Signaling server error: {}", e);
        }
    });

    println!("AStream PC ready. Signaling on ws://0.0.0.0:8080");
    println!("Waiting for connections...");

    loop {
        sleep(Duration::from_secs(1)).await;
    }
}
