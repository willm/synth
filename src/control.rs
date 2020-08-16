use super::{create_midi_connection, midi_to_freq, MidiMessage};
use std::sync::mpsc;

pub fn alternate_tones(
  synth_sender: std::sync::mpsc::Sender<[f32; 3]>,
) -> Result<(), Box<dyn std::error::Error>> {
  loop {
    synth_sender.send([300.0, 303.0, 299.0])?;
    std::thread::sleep(std::time::Duration::from_secs(3));
    synth_sender.send([600.0, 603.0, 599.0])?;
    std::thread::sleep(std::time::Duration::from_secs(3));
  }
}

pub fn read_midi_input(
  synth_sender: std::sync::mpsc::Sender<[f32; 3]>,
) -> Result<(), Box<dyn std::error::Error>> {
  let (tx, rx) = mpsc::channel::<MidiMessage>();
  let _connection = create_midi_connection(tx)?;

  loop {
    match rx.recv()? {
      MidiMessage::NoteOn { note, .. } => {
        let freq = midi_to_freq(note);
        println!("NOTE ON midi note {} {}Hz", note, freq);
        synth_sender.send([freq, freq + 3.0, freq - 1.0])?;
      }
      MidiMessage::NoteOff { note } => {
        println!("NOTE OFF midi note {} {}Hz", note, midi_to_freq(note));
        //synth_sender.send([0.0, 0.0, 0.0])?;
      }
    };
  }
}
