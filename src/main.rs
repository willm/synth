extern crate anyhow;
extern crate cpal;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::mpsc;

fn main() -> Result<(), anyhow::Error> {
    let (tx, rx) = mpsc::channel::<f32>();
    std::thread::spawn(move || {
        println!("spawned thread");
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .expect("failed to find a default output device");
        let config = device.default_output_config().unwrap();

        match config.sample_format() {
            cpal::SampleFormat::F32 => run::<f32>(&device, &config.into(), rx).unwrap(),
            cpal::SampleFormat::I16 => run::<i16>(&device, &config.into(), rx).unwrap(),
            cpal::SampleFormat::U16 => run::<u16>(&device, &config.into(), rx).unwrap(),
        }
    });

    loop {
        let mut freq: String = String::new();
        std::io::stdin()
            .read_line(&mut freq)
            .expect("Failed to read stdin");
        //print!("The freq is {}", freq);
        let ifreq = str::parse::<f32>(&freq[..freq.len() - 1]).unwrap();
        tx.send(ifreq).unwrap();
    }

    Ok(())
}

fn run<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    rx: std::sync::mpsc::Receiver<f32>,
) -> Result<(), anyhow::Error>
where
    T: cpal::Sample,
{
    let sample_rate = config.sample_rate.0 as f32;
    let channels = config.channels as usize;

    // Produce a sinusoid of maximum amplitude.
    let mut sample_clock = 0f32;
    let pi = std::f32::consts::PI;
    let mut freq = 440.0;
    let mut next_value = move || {
        freq = match rx.try_recv() {
            // try_recv asynchronously tries to get a value without blocking
            Ok(v) => v,
            _ => freq,
        };
        sample_clock = (sample_clock + 1.0) % sample_rate;
        (sample_clock * freq * 2.0 * pi / sample_rate).sin()
    };

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            write_data(data, channels, &mut next_value)
        },
        err_fn,
    )?;
    stream.play()?;
    loop {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

fn write_data<T>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> f32)
where
    T: cpal::Sample,
{
    for frame in output.chunks_mut(channels) {
        let value: T = cpal::Sample::from::<f32>(&next_sample());
        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}
