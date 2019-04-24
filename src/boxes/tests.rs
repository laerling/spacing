#![cfg(test)]
use boxes::{Boxes, BOX_DEFAULT_CAPACITY, Entry, SelectedEntry};
use std::fs::File;
use std::io::{Result, Write, BufRead, BufReader};

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
    let e: Entry = "foo = bar".parse().expect("Couldn't parse string into Entry");
    assert_eq!(e.lhs, "foo");
    assert_eq!(e.rhs, "bar");
}

#[test]
fn parse_erroneous_entry() {
    "erroneous entry".parse::<Entry>().expect_err("Erroneous entry was passed without expected error");
}

// FIXME Create /tmp/spacing__new_boxes_from_files/ first or empty it if it already exists
#[test]
fn new_boxes_from_files() {

    // create files
    for i in 0..5 {
        let mut box_file = File::create(format!("/tmp/box{}", i+1).as_str()).expect("Can't create box files for test");
        box_file.write(b"foo = bar").expect("Can't write to box files for test");
    }

    // create boxes
    let b = Boxes::from_files(&String::from("/tmp"));

    // check boxes
    for i in 0..5 {
        assert_eq!(b.boxes[i].len(), 1);
        assert_eq!(b.boxes[i][0], Entry { lhs: String::from("foo"), rhs: String::from("bar") });
    }
}

// FIXME Create /tmp/spacing__save_boxes/ first or empty it if it already exists
#[test]
fn save_boxes() {

    // create boxes
    let mut b = Boxes::new();
    for i in 0..5 {
        // write multiple entries in order to test if multiple lines are written correctly
        for _ in 0..i+1 {
            b.boxes[i].push(Entry { lhs: String::from("foo"), rhs: String::from("bar") });
        }
    }

    // save
    b.save(&String::from("/tmp"));

    // check files
    for i in 0..5 {
        let box_file = File::open(format!("/tmp/box{}", i+1).as_str()).expect("Can't open box file");
        let lines: Vec<String> = BufReader::new(box_file).lines().collect::<Result<_>>().expect("Cannot read lines from box file");
        assert_eq!(lines.len(), i+1);
        for line in lines {
            assert_eq!(line, "foo = bar");
        }
    }
}

#[test]
fn select_entry() {

    // create entry
    let e = Entry { lhs: String::from("foo"), rhs: String::from("bar") };

    // create boxes
    let mut b = Boxes::new();
    b.boxes[2].push(e.clone());

    // select and compare
    let selected = b.select_entry(2, 0);
    assert_eq!(selected.lhs, e.lhs);
    assert_eq!(selected.rhs, e.rhs);
}

#[test]
fn move_entry() {

    // create entries
    let e1 = Entry { lhs: String::from("e1 lhs"), rhs: String::from("e1 lhs") };
    let e2 = Entry { lhs: String::from("e2 lhs"), rhs: String::from("e2 lhs") };

    // create boxes
    let mut b = Boxes::new();
    b.boxes[0].push(e1.clone());
    b.boxes[4].push(e2.clone());

    // check left box boundary
    let e = e1.clone();
    b.move_entry(SelectedEntry { lhs: e.lhs, rhs: e.rhs, box_i: 0, entry_i: 0 }, false);
    assert_eq!(b.boxes[0].len(), 1); // entry still in 0...
    assert_eq!(b.boxes[1].len(), 0); // and not moved to 1
    assert_eq!(b.boxes[0][0], e1);

    // check moving backward
    let e = e2.clone();
    b.move_entry(SelectedEntry { lhs: e.lhs, rhs: e.rhs, box_i: 4, entry_i: 0 }, false);
    assert_eq!(b.boxes[4].len(), 0); // element moved from 4...
    assert_eq!(b.boxes[3].len(), 1); // to 3
    assert_eq!(b.boxes[3][0], e2);

    // check moving forward
    let e = e2.clone();
    b.move_entry(SelectedEntry { lhs: e.lhs, rhs: e.rhs, box_i: 3, entry_i: 0 }, true);
    assert_eq!(b.boxes[3].len(), 0); // element moved from 3...
    assert_eq!(b.boxes[4].len(), 1); // to 4
    assert_eq!(b.boxes[4][0], e2);

    // check right box boundary
    let e = e2.clone();
    b.move_entry(SelectedEntry { lhs: e.lhs, rhs: e.rhs, box_i: 4, entry_i: 0 }, true);
    assert_eq!(b.boxes[4].len(), 0); // entry deleted
    assert_eq!(b.boxes[3].len(), 0); // and not moved to 3
}
