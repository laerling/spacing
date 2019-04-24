mod boxes;

use boxes::Boxes;
use std::env::args;
use std::io::{BufRead, BufReader, stdin, stdout, Write};


fn main() {

    // used later for saving back the boxes
    let boxes_dir = args().skip(1).next();

    // load or create boxes
    let mut boxes = match &boxes_dir {
        Some(dir) => Boxes::from_files(dir),
        None => Boxes::new(),
    };

    // main loop
    loop {

        // select entry
        let entry = match boxes.select_random_entry() {
            Some(e) => e,
            None => {
                println!("Cannot select entry from any box. Are the boxes empty?");
                break;
            },
        };

        // ask user
        let question = entry.lhs.clone();
        let answer = entry.rhs.clone();
        let mut user_answer = String::new();
        print!("Box {}: {} = ", entry.box_i+1, question);
        stdout().flush().expect("Could not flush output");
        BufReader::new(stdin()).read_line(&mut user_answer).expect("Non-UTF-8 character read");

        // end loop if input is empty
        if user_answer.trim_end().is_empty() { break; }

        // check answer
        let correct = String::from(user_answer.trim_end()) == answer;
        if correct {
            println!("Correct!");
        } else {
            println!("Wrong. {} = {}", question, answer);
        }

        // move entry according to answer of user
        boxes.move_entry(entry, correct);
    }

    // end
    match &boxes_dir {
        Some(dir) => boxes.save(dir),
        None => boxes.save(&String::from(".")),
    }
}
