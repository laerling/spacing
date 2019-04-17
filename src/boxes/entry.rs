use std::str::FromStr;
use std::fmt::{Debug, Formatter};


pub struct Entry {
    lhs: String,
    rhs: String,
}

pub struct SideMissingError {}

impl Debug for SideMissingError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        f.write_str("One side missing (marker ' = ' not found)")
    }
}

impl FromStr for Entry {
    type Err = SideMissingError;

    fn from_str(s: &str) -> Result<Entry, <Entry as FromStr>::Err> {

        // split
        let sides: Vec<&str> = s.split(" = ").collect();

        // check that there are two sides
        if sides.len() < 2 {
            return Err(SideMissingError{});
        }

        Ok(Entry {
            lhs: String::from(sides[0]),
            rhs: String::from(sides[1]),
        })
    }
}
