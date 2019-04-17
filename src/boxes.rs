mod entry;
use self::entry::Entry;

use std::fs::{read_dir, ReadDir};

const BOXES_DEFAULT_CAPACITY: usize = 10; // recommended: two-digit number for small vocabulary, three-digit number for intermediate to big vocabulary
const BOX_FILE_PREFIX: &str = "box";

pub struct Boxes(Vec<Entry>, Vec<Entry>, Vec<Entry>, Vec<Entry>, Vec<Entry>);

impl Boxes {
    pub fn new() -> Boxes {
        return Boxes(
            Vec::with_capacity(BOXES_DEFAULT_CAPACITY),
            Vec::with_capacity(BOXES_DEFAULT_CAPACITY),
            Vec::with_capacity(BOXES_DEFAULT_CAPACITY),
            Vec::with_capacity(BOXES_DEFAULT_CAPACITY),
            Vec::with_capacity(BOXES_DEFAULT_CAPACITY),
        );
    }

    pub fn from_files(dir: &String) -> Boxes {

        // check presence of box files
        let items: ReadDir = read_dir(dir).expect(format!("Cannot read directory {}", dir).as_str());
        let box_files = items.filter_map(|item| {
            let item = item.expect("Cannot read item");
            let file_name = item.file_name().into_string().expect("File name is not valid UTF-8");

            // check filename
            for n in 1..5 {
                if file_name == format!("{}{}", BOX_FILE_PREFIX, n) {
                    return Some(item);
                }
            }

            // filename does not match
            None
        });

        // TODO
        return Boxes::new();
    }

    /**
     * If dir is None, use current directory for saving
     **/
    pub fn save(&self, dir: &Option<String>) {
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
