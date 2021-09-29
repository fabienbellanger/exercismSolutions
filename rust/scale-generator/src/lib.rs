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
struct Note {
    base: Root,
    alteration: Alteration,
}

#[derive(Debug)]
pub struct Scale {
    Tonic: Note,
    Intervals: Vec<Interval>,
}

/* TODO: Pas bon à reprendre
 0: [A, Bb]
 1: [B, A#]
 2: [B#, Cb]
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
    fn base_interval() -> HashMap<(Note, Note), Interval> {
        let mut notes = HashMap::new();

        notes.insert(
            (
                Note {
                    base: Root::A,
                    alteration: Alteration::None,
                },
                Note {
                    base: Root::B,
                    alteration: Alteration::None,
                },
            ),
            Interval::Major,
        );

        notes
    }

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
