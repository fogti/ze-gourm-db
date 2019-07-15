extern crate ed_it;
extern crate spawn_editor;

use std::fs::File;

fn main() {
    let f = File::open("testfile.txt").expect("file-open failed");
    let r = ed_it::parse_recipe_from_file(f).expect("parsing failed");
    println!("{:#?}", &r);
    println!("{}", r.to_string(std::collections::HashMap::new()));
}
