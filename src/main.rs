use anyhow::Result;
use cpal::Device;
use rodio::Source;
use std::fs::File;
use std::io::{stdin, stdout, BufReader, Write};
use std::path::Path;
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() -> Result<()> {
    let device = rodio::default_output_device().unwrap();

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode()?;

    write!(
        stdout,
        "{}{}Esc to exit. Type stuff, use alt, and so on.{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Hide
    )?;
    stdout.flush()?;

    for c in stdin.events() {
        match c? {
            Event::Key(Key::Esc) => break,
            Event::Key(Key::Char('a')) => play_file(&device, "sound/op.ogg")?,
            _ => (),
        }
    }

    write!(stdout, "{}", termion::cursor::Show)?;

    Ok(())
}

fn play_file<P: AsRef<Path>>(device: &Device, path: P) -> Result<()> {
    let file = File::open(path)?;
    let source = rodio::Decoder::new(BufReader::new(file))?;
    rodio::play_raw(&device, source.convert_samples());
    Ok(())
}
