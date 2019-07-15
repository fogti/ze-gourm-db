extern crate ed_it;
extern crate spawn_editor;

use std::fs::File;

fn main() {
    let f = File::open("testfile.txt").expect("file-open failed");
    println!("{:#?}", ed_it::parse_recipe_from_file(f).expect("parsing failed"));
}
