use std::thread::sleep;

use anyhow::Result;
use midir::{ConnectError, MidiOutput, MidiOutputConnection};
use tracing::info;

use munote::chord::Chord;
// use munote::duration::Duration;
use munote::note::Note;
use munote::rest::Rest;
use munote::tag::Tag;
use munote::visitor::Visitor;
use munote::voice::Voice;

pub struct PlaybackContext {
    output: MidiOutputConnection,
    tempo: f32,
}

impl PlaybackContext {
    pub fn new(tempo: f32) -> Result<Self> {
        let midi_out = MidiOutput::new("MIDI Output")?;

        let out_ports = midi_out.ports();

        println!("\nAvailable output ports:");
        for (i, p) in out_ports.iter().enumerate() {
            println!("{}: {}", i, midi_out.port_name(p).unwrap());
        }

        let out_port = &out_ports[0];
        let out_conn = midi_out.connect(out_port, "MIDI Output Connection")
            .map_err(|e| ConnectError::new(e.kind(), ()))?;

        Ok(Self {
            tempo,
            output: out_conn,
        })
    }
}

impl Visitor for PlaybackContext {
    fn on_chord(&mut self, _chord: &Chord) {
        todo!()
    }

    fn on_note(&mut self, note: &Note) {
        let mut play_note = |note: u8, duration: f32| {
            const NOTE_ON_MSG: u8 = 0x90;
            const NOTE_OFF_MSG: u8 = 0x80;
            const VELOCITY: u8 = 0x64;

            // We're ignoring errors in here
            let _ = self.output.send(&[NOTE_ON_MSG, note, VELOCITY]);
            let duration_ms = self.tempo * duration * 4.0;
            info!("Duration for {:?}: {:?} ({}ms)", note, duration, duration_ms);

            sleep(std::time::Duration::from_millis(duration_ms as u64));
            let _ = self.output.send(&[NOTE_OFF_MSG, note, VELOCITY]);
        };

        play_note((note.chromatic_pitch() + 66) as u8, note.full_duration().as_f32());
    }

    fn on_rest(&mut self, rest: &Rest) {
        let duration_ms = self.tempo * rest.full_duration().as_f32() * 4.0;
        sleep(std::time::Duration::from_millis(duration_ms as u64));
    }

    fn on_staff_begin(&mut self) {}

    fn on_staff_end(&mut self) {}

    fn on_tag(&mut self, _tag: &Tag) {}

    fn on_voice(&mut self, _voice: &Voice) {}
}