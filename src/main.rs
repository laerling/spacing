extern crate rand;

mod boxes;

use boxes::Boxes;
use std::env::args;


fn main() {

    // used later for saving back the boxes
    let boxes_dir = args().skip(1).next();

    // load or create boxes
    let mut boxes = match &boxes_dir {
        Some(dir) => Boxes::from_files(dir).expect("Could not open box files"),
        None => Boxes::new(),
    };

    // main loop
    // TODO insert break condition
    for _ in 0..3 {
        let entry = match boxes.select_random_entry() {
            Some(e) => e,
            None => break,
        };

        // TODO ask user

        // move entry according to answer of user
        boxes.move_entry(entry, true);
    }

    // end
    match &boxes_dir {
        Some(dir) => boxes.save(dir),
        None => boxes.save(&String::from(".")),
    }
}
