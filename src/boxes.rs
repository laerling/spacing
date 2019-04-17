mod entry;
use self::entry::Entry;

use std::fs::{read_dir, ReadDir, File};
use std::io::{Result, BufRead, BufReader};

const BOX_DEFAULT_CAPACITY: usize = 10; // recommended: two-digit number for small vocabulary, three-digit number for intermediate to big vocabulary
const BOX_FILE_PREFIX: &str = "box";

pub struct Boxes {
    boxes: [Vec<Entry>; 5],
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
            let file_name = item.file_name().into_string().expect("File name is not valid UTF-8");

            // check filename
            for i in 1..5 {
                if file_name == format!("{}{}", BOX_FILE_PREFIX, i) {
                    return Some(file_name);
                }
            }

            // filename does not match
            None
        }).collect();

        // check that all box files have been found
        if box_filenames.len() != 5 {
            return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Not all box files found"));
        }

        // make boxes object
        let mut boxes = Boxes::new();
        for i in 0..4 {
            let box_file = File::open(box_filenames[i].as_str()).expect(format!("Can't open file {}", box_filenames[i]).as_str());

            // parse file contents
            for line in BufReader::new(box_file).lines() {
                let line = line.expect("Cannot read line from file");
                let entry: Entry = line.parse().expect("Parser error");
                boxes.boxes[i].push(entry);
            }
        }

        Ok(boxes)
    }

    pub fn save(&self, dir: &Option<String>) {
        // If dir is None, use current directory for saving
        let dir = match dir {
            Some(d) => d,
            None => ".",
        }

        // TODO
    }

    /**
     * Returns true if the main loop has to be exited
     **/
    pub fn round(&self) -> bool {
        // TODO
        true
    }
}
