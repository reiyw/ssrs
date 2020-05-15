use anyhow::Result;
use ssrs::SoundList;
use std::io::{stdin, stdout, Write};
use std::path::PathBuf;
use structopt::StructOpt;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

#[derive(StructOpt, Debug)]
#[structopt(name = "ssrs")]
struct Opt {
    /// Directory that stores sound files
    #[structopt(name = "DIR", parse(from_os_str), default_value = ".")]
    dir: PathBuf,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    let sl = SoundList::from_directory(opt.dir);

    let device = rodio::default_output_device().unwrap();

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode()?;

    write!(
        stdout,
        "{}{}Esc to exit.{}\n\r",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Hide
    )?;
    stdout.flush()?;

    for (key, sound) in sl.iter() {
        write!(stdout, "{}) {}\n\r", key, sound)?;
    }

    for key in stdin.keys() {
        match key? {
            Key::Esc => break,
            Key::Char(c) => {
                if let Some(sound) = sl.get_sound_from_key(c) {
                    sound.play(&device)?;
                }
            }
            _ => (),
        }
    }

    write!(stdout, "{}", termion::cursor::Show)?;

    Ok(())
}
