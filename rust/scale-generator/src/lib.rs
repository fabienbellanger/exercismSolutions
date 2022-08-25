const SHARPS: &[&str] = &[
    "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#",
];
const FLATS: &[&str] = &[
    "A", "Bb", "B", "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab",
];

const USE_SHARPS: &[&str] = &[
    "C", "a", "G", "D", "A", "E", "B", "F#", "e", "b", "f#", "c#", "g#", "d#",
];
const USE_FLATS: &[&str] = &[
    "C", "a", "F", "Bb", "Eb", "Ab", "Db", "Gb", "d", "g", "c", "f", "bb", "eb",
];

#[derive(Debug)]
pub enum Error<'a> {
    InvalidTonic(&'a str),
    InvalidScale(char),
}

#[derive(Debug)]
pub struct Scale {
    notes: Vec<String>,
}

impl Scale {
    pub fn new<'a>(tonic: &'a str, intervals: &str) -> Result<Scale, Error<'a>> {
        let chromotic_scale = if USE_SHARPS.contains(&tonic) {
            SHARPS
        } else if USE_FLATS.contains(&tonic) {
            FLATS
        } else {
            return Err(Error::InvalidTonic(tonic));
        };

        let mut position_in_scale = chromotic_scale
            .iter()
            .position(|&note| note.to_uppercase() == tonic.to_uppercase())
            .ok_or(Error::InvalidTonic(tonic))?;

        let mut notes = vec![chromotic_scale[position_in_scale].to_string()];
        let scale_length = chromotic_scale.len();

        for interval in intervals.chars() {
            let step = match interval {
                'm' => 1,
                'M' => 2,
                'A' => 3,
                _ => return Err(Error::InvalidScale(interval)),
            };
            position_in_scale = (position_in_scale + step) % scale_length;

            notes.push(chromotic_scale[position_in_scale].to_string());
        }

        Ok(Self { notes })
    }

    pub fn chromatic(tonic: &str) -> Result<Scale, Error> {
        Self::new(tonic, "mmmmmmmmmmmm")
    }

    pub fn enumerate(&self) -> Vec<String> {
        self.notes.clone()
    }
}
