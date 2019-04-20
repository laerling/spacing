extern crate rand;

mod entry;
mod tests;

use self::rand::Rng;
use self::rand::distributions::{Distribution, LogNormal};
use self::entry::Entry;
use std::fs::{read_dir, ReadDir, File};
use std::io::{BufRead, BufReader, Write, stdin, stdout};
use std::path::Path;

const BOX_DEFAULT_CAPACITY: usize = 10; // recommended: two-digit number for small vocabulary, three-digit number for intermediate to big vocabulary
const BOX_FILE_PREFIX: &str = "box";

pub struct Boxes {
    boxes: [Vec<Entry>; 5],
    finished: Vec<Entry>,
}

/**
 * Holds state about a selected entry so that it can be supplied to other methods of Boxes.
 * This struct only makes sense in the context of a collection of boxes, so it's implemented here, 
 * rather than in the Entry module.
 **/
pub struct SelectedEntry {
    pub lhs: String,
    pub rhs: String,
    pub box_i: usize,
    entry_i: usize,
}

/**
 * box_i is 0-indexed
 **/
fn box_name(box_i: usize) -> String {
    format!("{}{}", BOX_FILE_PREFIX, box_i + 1)
}

impl Boxes {
    pub fn new() -> Boxes {
        Boxes {
            boxes: [
                Vec::with_capacity(BOX_DEFAULT_CAPACITY),
                Vec::with_capacity(BOX_DEFAULT_CAPACITY),
                Vec::with_capacity(BOX_DEFAULT_CAPACITY),
                Vec::with_capacity(BOX_DEFAULT_CAPACITY),
                Vec::with_capacity(BOX_DEFAULT_CAPACITY),
            ],
            finished: Vec::with_capacity(BOX_DEFAULT_CAPACITY),
        }
    }

    pub fn from_files(dir: &String) -> Boxes {

        // find box files
        let items: ReadDir = read_dir(dir).expect(format!("Cannot read directory {}", dir).as_str());
        let box_filenames: Vec<String> = items.filter_map(|item| {
            let item = item.expect("Cannot read item");
            let filename = item.file_name().into_string().expect("File name is not valid UTF-8");

            // check filename
            for box_i in 0..5 {
                if filename == box_name(box_i) {
                    return Some(filename);
                }
            }

            // filename does not match
            None
        }).collect();

        // ask user whether to create missing box files
        if box_filenames.len() != 5 {

            // ask user
            println!("At least one box file doesn't exist in {}", dir);
            print!("Create missing box files? (N/y)");
            stdout().flush().expect("Could not flush output");
            let mut input = String::new();
            BufReader::new(stdin()).read_line(&mut input).expect("Non-UTF-8 character read");

            // create missing box files or terminate program
            if input.to_ascii_lowercase().starts_with("y") {

                // create missing boxes
                for i in 0..5 {
                    let box_name = box_name(i);

                    // skip existing boxes
                    if box_filenames.contains(&box_name) { continue; }

                    // create box
                    File::create(Path::new(dir).join(box_name).as_path()).expect("Can't create box file");
                }
            } else {

                // terminate program
                println!("Terminating");
                std::process::exit(0);
            }
        }

        // box_filenames was only for checking the presence of the box files and shouldn't be used
        // anymore, especially since there is no guarantee about the order of its elements.
        std::mem::drop(box_filenames);

        // make boxes object
        let mut boxes = Boxes::new();
        for box_i in 0..5 {
            let filename = Path::new(dir.as_str()).join(box_name(box_i).as_str());
            let file = File::open(filename.as_path()).expect(format!("Can't open file {}", filename.as_path().display()).as_str());

            // parse file contents
            for line in BufReader::new(file).lines() {
                let line = line.expect("Cannot read line from file");
                let entry: Entry = line.parse().expect("Parser error");
                boxes.boxes[box_i].push(entry);
            }
        }

        boxes
    }

    fn save_box(&self, filename: &Path, entries: &Vec<Entry>) {

            // open file
            let mut file = File::create(filename).expect(format!("Cannot write file '{}'", filename.display()).as_str());

            // write contents
            let error_msg = format!("Cannot write to file {}", filename.display());
            for entry_i in 0..entries.len() {
                if entry_i != 0 { file.write(b"\n").expect(error_msg.as_str()); }
                let entry = &entries[entry_i];
                file.write(format!("{} = {}", entry.lhs, entry.rhs).as_bytes()).expect(error_msg.as_str());
            }
    }

    pub fn save(&self, dir: &String) {

        // save boxes
        for box_i in 0..5 {
            let filename = Path::new(dir).join(box_name(box_i).as_str());
            self.save_box(&filename.as_path(), &self.boxes[box_i]);
        }

        // save finished entries
        let filename = Path::new(dir).join("finished");
        self.save_box(&filename.as_path(), &self.finished);
    }

    /**
     * return value is 0-indexed
     **/
    pub fn select_random_box() -> usize {

        // initialize RNG
        let mut rng = rand::thread_rng();
        let d = LogNormal::new(0.0, 1.0);

        // don't count too high numbers towards last box
        let mut r: f64 = 6.0;
        while r >= 6.0 {
            r = d.sample(&mut rng);
        }

        // choose box
        for i in 0..5 {
            let box_n: usize = 4 - i;
            if r >= f64::from(box_n as u16) {
                return box_n;
            }
        }
        return 0;
    }

    /**
     * return value is 0-indexed
     **/
    pub fn select_random_nonempty_box(&self) -> Option<usize> {

        // check if there even are any entries to select from
        if self.boxes.iter().all(|b| { b.is_empty() }) {
            return None;
        }

        // select non-empty box
        loop {
            let box_i = Boxes::select_random_box();
            if !self.boxes[box_i].is_empty() {
                return Some(box_i);
            }
        }
    }

    pub fn select_random_entry(&self) -> Option<SelectedEntry> {

        // select box
        let box_i = match self.select_random_nonempty_box() {
            Some(i) => i,
            None => return None,
        };

        // select entry from box
        let mut rng = rand::thread_rng();
        let entry_i = rng.gen_range(0, self.boxes[box_i].len());

        // return selected entry
        Some(self.select_entry(box_i, entry_i))
    }

    /**
     * Select an entry from a specific point in a specific box.
     **/
    pub fn select_entry(&self, box_i: usize, entry_i: usize) -> SelectedEntry {

        // select
        let e = &self.boxes[box_i][entry_i];

        // build selected entry
        SelectedEntry {
            lhs: e.lhs.clone(),
            rhs: e.rhs.clone(),
            box_i: box_i,
            entry_i: entry_i,
        }
    }

    /**
     * Move an entry one box further if it's been answered correctly, else one box back.
     * If the entry is in the last box and was answered correctly, delete it.
     * If the entry is in the first box and was not answered correctly, don't move it.
     **/
    pub fn move_entry(&mut self, e: SelectedEntry, successful: bool) {

        // panic if entry doesn't exist. This shouldn't happen because the fields of SelectedEntry 
        // are private and thus can't be changed outside of methods of Boxes.
        let expected_entry = Entry { lhs: e.lhs, rhs: e.rhs };
        if self.boxes[e.box_i][e.entry_i] != expected_entry {
            panic!("Expected entry does not exist");
        }

        // Call remove after push, because we'd rather be left in an erroneous state where we have 
        // twice the same entry, than having none at all.
        if successful {
            // move forward
            if e.box_i == 4 {
                self.finished.push(expected_entry);
            } else {
                self.boxes[e.box_i+1].push(expected_entry);
            }
            self.boxes[e.box_i].remove(e.entry_i);
            return;

        } else if !successful && e.box_i > 0 {
            // move backward
            self.boxes[e.box_i-1].push(expected_entry);
            self.boxes[e.box_i].remove(e.entry_i);
            return;
        }
    }
}
