#[derive(Debug)]
pub struct AudioConfig {
    pub sample_rate: u32,
    pub channels: u16,
    pub buffer_size: u32,
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            sample_rate: 48000,
            channels: 2,
            buffer_size: 960, // 20ms @ 48kHz
        }
    }
}

pub fn get_default_input_device() -> Option<String> {
    Some("Default Input (Stub)".to_string())
}

pub fn get_default_output_device() -> Option<String> {
    Some("Default Output (Stub)".to_string())
}

pub struct AudioStream;

impl AudioStream {
    pub fn new() -> Self {
        AudioStream
    }
    
    pub fn start(&self) -> Result<(), String> {
        println!("Audio stream started (stub)");
        Ok(())
    }
}
