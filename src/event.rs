
use std::fmt;

pub enum Event {
    VimpointScratchPad,

    UNKNOWN,
}

impl fmt::Debug for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Event::Happened");
    }
}

