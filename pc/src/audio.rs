use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{BufferSize, SampleFormat, StreamConfig};

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

pub fn get_default_input_device() -> Option<cpal::Device> {
    let host = cpal::default_host();
    host.default_input_device()
}

pub fn get_default_output_device() -> Option<cpal::Device> {
    let host = cpal::default_host();
    host.default_output_device()
}

pub fn create_input_stream<F>(device: &cpal::Device, config: &AudioConfig, mut callback: F) -> Result<cpal::Stream, cpal::BuildStreamError>
where
    F: FnMut(&[f32]) + Send + 'static,
{
    let stream_config = StreamConfig {
        channels: config.channels,
        sample_rate: cpal::SampleRate(config.sample_rate),
        buffer_size: BufferSize::Fixed(config.buffer_size),
    };

    let err_fn = |err| eprintln!("Stream error: {}", err);

    device.build_input_stream(
        &stream_config,
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            callback(data);
        },
        err_fn,
        None,
    )
}

pub fn create_output_stream<F>(device: &cpal::Device, config: &AudioConfig, mut callback: F) -> Result<cpal::Stream, cpal::BuildStreamError>
where
    F: FnMut(&mut [f32]) + Send + 'static,
{
    let stream_config = StreamConfig {
        channels: config.channels,
        sample_rate: cpal::SampleRate(config.sample_rate),
        buffer_size: BufferSize::Fixed(config.buffer_size),
    };

    let err_fn = |err| eprintln!("Stream error: {}", err);

    device.build_output_stream(
        &stream_config,
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            callback(data);
        },
        err_fn,
        None,
    )
}
