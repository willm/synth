use std::error::Error;
use synth::{control, core};

fn main() -> Result<(), Box<dyn Error>> {
    let synth_sender = core::start_synth();

    if std::env::args().any(|arg| arg == "--midi-input") {
        control::read_midi_input(synth_sender)
    } else {
        control::alternate_tones(synth_sender)
    }
}
