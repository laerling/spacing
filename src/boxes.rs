mod entry;
use self::entry::Entry;

use std::fs::{read_dir, ReadDir, File};
use std::io::{Result, BufRead, BufReader, Write};
use std::path::Path;

const BOX_DEFAULT_CAPACITY: usize = 10; // recommended: two-digit number for small vocabulary, three-digit number for intermediate to big vocabulary
const BOX_FILE_PREFIX: &str = "box";

pub struct Boxes {
    boxes: [Vec<Entry>; 5],
}

/**
 * Holds state about a selected entry so that it can be supplied to other methods of Boxes
 **/
pub struct SelectedEntry {
    lhs: String,
    rhs: String,
    box_i: usize,
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
        Boxes { boxes: [
            Vec::with_capacity(BOX_DEFAULT_CAPACITY),
            Vec::with_capacity(BOX_DEFAULT_CAPACITY),
            Vec::with_capacity(BOX_DEFAULT_CAPACITY),
            Vec::with_capacity(BOX_DEFAULT_CAPACITY),
            Vec::with_capacity(BOX_DEFAULT_CAPACITY),
        ]}
    }

    pub fn from_files(dir: &String) -> Result<Boxes> {

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

        // check that all box files have been found
        if box_filenames.len() != 5 {
            return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Not all box files found"));
        }

        // box_filesnames was for only for checking the presence of the box files and shouldn't be
        // used anymore, especially since there is no guarantee about the order of its elements.
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

        Ok(boxes)
    }

    pub fn save(&self, dir: &Option<String>) {
        // If dir is None, use current directory for saving
        let dir = match dir {
            Some(d) => Path::new(d.as_str()),
            None => Path::new("."),
        };

        // write every box
        for box_i in 0..5 {

            // open file
            let filename = dir.join(box_name(box_i).as_str());
            let mut file = File::create(filename.as_path()).expect(format!("Cannot write to file {}", filename.as_path().display()).as_str());

            // write contents
            let error_msg = format!("Cannot write to file {}", filename.as_path().display());
            let entries = &self.boxes[box_i];
            for entry_i in 0..entries.len() {
                if entry_i != 0 {
                    file.write(b"\n").expect(error_msg.as_str());
                }
                let entry = &entries[entry_i];
                file.write(format!("{} = {}", entry.lhs, entry.rhs).as_bytes()).expect(error_msg.as_str());
            }
        }
    }

    pub fn select_random_entry(&self) -> SelectedEntry {
        // TODO select box, select entry from box
        SelectedEntry {
            lhs: String::from(""),
            rhs: String::from(""),
            box_i: 0,
            entry_i: 0,
        }
    }

    /**
     * Move an entry one box further if it's been answered correctly, else one box back.
     * If the entry is in the last box and was answered correctly, delete it.
     * If the entry is in the first box and was not answered correctly, don't move it.
     **/
    pub fn move_entry(&self, e: SelectedEntry, successful: bool) {

        // panic if entry doesn't exist. This shouldn't happen because the fields of SelectedEntry 
        // are private and thus can't be changed outside of methods of Boxes.
        let expected_entry = Entry { lhs: e.lhs, rhs: e.rhs };
        if self.boxes[e.box_i][e.entry_i] != expected_entry {
            panic!("Expected entry does not exist");
        }

        // Call remove after push, because we'd rather be left in an erroneous state where we have 
        // twice the same entry, than having none at all.
        if successful && e.box_i < 4 {
            // move forward
            self.boxes[e.box_i+1].push(expected_entry);
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
