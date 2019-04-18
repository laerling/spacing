use boxes::{Boxes, BOX_DEFAULT_CAPACITY, Entry};
use std::fs::File;
use std::io::Write;

#[test]
fn new_boxes_statelessnes() {
    let b1 = Boxes::new();
    let b2 = Boxes::new();
    for b_i in 0..b1.boxes.len() {
        assert_eq!(b1.boxes[b_i], b2.boxes[b_i]);
    }
}

#[test]
fn new_boxes_default_capacity() {
    let b = Boxes::new();
    for b in b.boxes.iter() {
        assert_eq!(b.capacity(), BOX_DEFAULT_CAPACITY);
    }
}

#[test]
fn parse_entry() {
    panic!("TODO");
}

#[test]
fn new_boxes_from_files() {
    // We can't test for erroneous entries, cause they cause Boxes::from_files to panic.
    // They're tested in tests::parse_entry.
    for i in 0..5 {
        let mut box_file = File::create(format!("/tmp/box{}", i+1).as_str()).expect("Can't create box files for test");
        box_file.write(b"foo = bar").expect("Can't write to box files for test");
    }
    let b = Boxes::from_files(&String::from("/tmp")).expect("Can't create boxes from files");
    for i in 0..5 {
        assert_eq!(b.boxes[i].len(), 1);
        assert_eq!(b.boxes[i][0], Entry { lhs: String::from("foo"), rhs: String::from("bar") });
    }
}

#[test]
fn save_boxes() {
    panic!("TODO");
}

#[test]
fn select_random_entry_from_boxes() {
    // TODO select multiple times and check probability distribution
    panic!("TODO");
}

#[test]
fn move_entry() {
    // TODO check moving of both successful and unsuccessful entry
    panic!("TODO");
}
