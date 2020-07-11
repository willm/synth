use cpal::{EventLoop, OutputBuffer, StreamData, UnknownTypeOutputBuffer};
use rand::prelude::*;

fn white_noise(mut buffer: OutputBuffer<f32>) {
    let mut rng = rand::thread_rng();
    for elem in buffer.iter_mut() {
        *elem = rng.gen::<f32>(); // generates a float between 0 and 1
    }
}

fn main() {
    let event_loop = EventLoop::new();
    let device = cpal::default_output_device().expect("no output device available");
    let mut supported_formats_range = device
        .supported_output_formats()
        .expect("error while querying formats");
    let format = supported_formats_range
        .next()
        .expect("no supported format?!")
        .with_max_sample_rate();
    println!("{:?}", format);
    let stream_id = event_loop.build_output_stream(&device, &format).unwrap();
    event_loop.build_input_stream(&device, &format).unwrap();
    event_loop.play_stream(stream_id);
    event_loop.run(move |_stream_id, mut _stream_data| match _stream_data {
        StreamData::Output {
            buffer: UnknownTypeOutputBuffer::U16(_buffer),
        } => {
            panic!("Unsupported output buffer type U16");
        }
        StreamData::Output {
            buffer: UnknownTypeOutputBuffer::I16(_buffer),
        } => {
            panic!("Unsupported output buffer type I16");
        }
        StreamData::Output {
            buffer: UnknownTypeOutputBuffer::F32(buffer),
        } => {
            white_noise(buffer);
        }
        _ => (),
    });
}
