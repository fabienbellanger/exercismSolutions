use std::collections::HashMap;

// You should change this.
//
// Depending on your implementation, there are a variety of potential errors
// which might occur. They aren't checked by the test suite in order to
// allow the greatest freedom of implementation, but real libraries should
// provide useful, descriptive errors so that downstream code can react
// appropriately.
//
// One common idiom is to define an Error enum which wraps all potential
// errors. Another common idiom is to use a helper type such as failure::Error
// which does more or less the same thing but automatically.
#[derive(Debug)]
pub struct Error;

#[derive(Debug)]
enum Interval {
    // Minor: 1/2 step
    Minor,

    // Major: 1 step
    Major,

    // Augmented: 1 1/2 step
    Augmented,
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Alteration {
    None,
    Sharp,
    Flat,
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Root {
    A,
    B,
    D,
    E,
    F,
    G,
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Mode {
    Major, // Uppercase
    Minor, // Lowercase
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Note {
    base: Root,
    Mode: Mode,
    Alteration: Alteration,
}

#[derive(Debug)]
pub struct Scale {
    Tonic: Note,
    Intervals: Vec<Interval>,
}

/* TODO: Put in HashMap 48 entries
    "A" => Note{Root: A, Mode: Major, Alteration: None},
    "ab" => Note{Root: A, Mode: Minor, Alteration: Flat},
    ...
 */

/* TODO: Put in a const
 0: [A]
 1: [A#, Bb]
 2: [B, Cb]
 3: [C]
 4: [C#, Db]
 5: [D]
 6: [D#, Eb]
 7: [E]
 8: [F, E#]
 9: [F#, Gb]
10: [G]
11: [G#, Ab]
*/

impl Scale {
    pub fn new(tonic: &str, intervals: &str) -> Result<Scale, Error> {
        unimplemented!(
            "Construct a new scale with tonic {} and intervals {}",
            tonic,
            intervals
        )
    }

    pub fn chromatic(tonic: &str) -> Result<Scale, Error> {
        unimplemented!("Construct a new chromatic scale with tonic {}", tonic)
    }

    pub fn enumerate(&self) -> Vec<String> {
        unimplemented!()
    }
}
