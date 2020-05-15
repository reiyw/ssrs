use anyhow::Result;
use cpal::Device;
use rodio::Source;
use std::collections::BTreeMap;
use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::iter::FromIterator;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

const KEYS: &[char] = &[
    'q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p', 'a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l',
    'z', 'x', 'c', 'v', 'b', 'n', 'm',
];
const AVAILABLE_EXTENSIONS: &[&str] = &["mp3", "wav", "wave", "ogg", "flac"];

#[derive(Debug)]
pub struct Sound {
    path: PathBuf,
}

impl Sound {
    pub fn play(&self, device: &Device) -> Result<()> {
        let file = File::open(&self.path)?;
        let source = rodio::Decoder::new(BufReader::new(file))?;
        rodio::play_raw(device, source.convert_samples());
        Ok(())
    }
}

impl fmt::Display for Sound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.path.display())
    }
}

impl<P: AsRef<Path>> From<P> for Sound {
    fn from(path: P) -> Self {
        Sound {
            path: path.as_ref().into(),
        }
    }
}

#[derive(Debug)]
pub struct SoundList {
    sounds: BTreeMap<char, Sound>,
}

impl SoundList {
    fn new() -> Self {
        Self {
            sounds: BTreeMap::new(),
        }
    }

    pub fn from_directory<P: AsRef<Path>>(dir: P) -> Self {
        WalkDir::new(dir)
            .into_iter()
            .map(|e| e.unwrap().into_path())
            .filter(|path| {
                path.extension().is_some()
                    && AVAILABLE_EXTENSIONS.contains(&path.extension().unwrap().to_str().unwrap())
            })
            .collect()
    }

    pub fn get_sound_from_key(&self, key: char) -> Option<&Sound> {
        self.sounds.get(&key)
    }

    pub fn iter(&self) -> std::collections::btree_map::Iter<char, Sound> {
        self.sounds.iter()
    }
}

impl<P: AsRef<Path>> FromIterator<P> for SoundList {
    fn from_iter<I: IntoIterator<Item = P>>(iter: I) -> Self {
        let mut sl = Self::new();
        for (&k, p) in KEYS.iter().zip(iter) {
            sl.sounds.insert(k, p.into());
        }
        sl
    }
}
