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

#[derive(Debug)]
enum Alteration {
    None,
    Sharp,
    Flat,
}

#[derive(Debug)]
struct Note {
    alteration: Alteration,
}

#[derive(Debug)]
pub struct Scale<'a> {
    Tonic: &'a str,
    Intervals: Vec<Interval>,
}

impl<'a> Scale<'a> {
    pub fn new(tonic: &str, intervals: &str) -> Result<Scale<'a>, Error> {
        unimplemented!(
            "Construct a new scale with tonic {} and intervals {}",
            tonic,
            intervals
        )
    }

    pub fn chromatic(tonic: &str) -> Result<Scale<'a>, Error> {
        unimplemented!("Construct a new chromatic scale with tonic {}", tonic)
    }

    pub fn enumerate(&self) -> Vec<String> {
        unimplemented!()
    }
}
