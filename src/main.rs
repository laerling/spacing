extern crate rand;

mod boxes;

use boxes::Boxes;
use std::env::args;
use std::io::{BufRead, BufReader, stdin};


fn main() {

    // used later for saving back the boxes
    let boxes_dir = args().skip(1).next();

    // load or create boxes
    let mut boxes = match &boxes_dir {
        Some(dir) => Boxes::from_files(dir).expect("Could not open box files"),
        None => Boxes::new(),
    };

    // main loop
    // TODO loop {
    for _ in 0..3 {
        let entry = match boxes.select_random_entry() {
            Some(e) => e,
            None => {
                println!("Cannot select entry from any box. Are the boxes empty?");
                break;
            },
        };

        // choose side, i. e. select question and answer
        let mut question: String;
        let mut answer: String;
        if rand::random() {
            question = entry.lhs.clone();
            answer = entry.rhs.clone();
        } else {
            question = entry.rhs.clone();
            answer = entry.lhs.clone();
        }

        // ask user
        let mut user_answer = String::new();
        print!("{} = ", question);
        BufReader::new(stdin()).read_line(&mut user_answer).expect("Non-UTF-8 character read");

        // move entry according to answer of user
        boxes.move_entry(entry, String::from(user_answer.trim_end()) == answer);
    }

    // end
    match &boxes_dir {
        Some(dir) => boxes.save(dir),
        None => boxes.save(&String::from(".")),
    }
}
