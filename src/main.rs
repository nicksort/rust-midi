extern crate midir;

use std::thread::sleep;
use std::time::Duration;
use std::io::{stdin, stdout, Write};
use std::error::Error;
use midir::{MidiOutput, MidiOutputPort};

fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err)
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let midi_out = MidiOutput::new("🎵🎵")?;
    let out_ports = midi_out.ports();
    let out_port: &MidiOutputPort = match out_ports.len() {
        0 => return Err("no output port found".into()),
        1 => {
            &out_ports[0]
        },
        2 => {
            &out_ports[0]
        },
        _ => {
            println!("\nAvailable output ports:");
            for (i, p) in out_ports.iter().enumerate() {
                println!("{}: {}", i, midi_out.port_name(p).unwrap());
            }
            print!("Please select output port: ");
            stdout().flush()?;
            let mut input = String::new();
            stdin().read_line(&mut input)?;
            out_ports.get(input.trim().parse::<usize>()?)
                     .ok_or("invalid output port selected")?
        }
    };

    let mut conn_out = midi_out.connect(out_port, "midir-test")?;
    {
        let mut play_note = |note: u8, duration: u64| {
            const NOTE_ON_MSG: u8 = 0x90;
            const NOTE_OFF_MSG: u8 = 0x80;
            const VELOCITY: u8 = 0x64;
            let _ = conn_out.send(&[NOTE_ON_MSG, note, VELOCITY]);
            sleep(Duration::from_millis(duration * 200));
            let _ = conn_out.send(&[NOTE_OFF_MSG, note, VELOCITY]);
        };

        // added a short midi loop for snow pads test
        for x in 0..4 {
            play_note(60, 3 + x);
            play_note(65, 3 + x);
            play_note(60, 2 + x);
            sleep(Duration::from_millis(200));
            play_note(60, 2 + x);
            play_note(65, 3 + x);
            play_note(65, 3 + x);
        }
    }
    conn_out.close();
    Ok(())
}
