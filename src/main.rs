#![allow(unused_variables, dead_code)] // dead_code for field of Entry struct

mod boxes;
use boxes::Boxes;

use std::env::args;


fn main() {

    // used later for saving back the boxes
    let boxes_dir = args().skip(1).next();

    // load or create boxes
    let boxes = match &boxes_dir {
        Some(dir) => Boxes::from_files(dir),
        None => Boxes::new(),
    };

    // main loop
    while boxes.round() { }

    // end
    boxes.save(&boxes_dir);
}
