extern crate linked_hash_set;
use linked_hash_set::LinkedHashSet;

fn main() {
    let intent = std::env::args().nth(1).expect("no pattern given");

    let sigil = intent
        .to_uppercase()
        .chars()
        .collect::<LinkedHashSet<char>>()
        .into_iter()
        .collect::<String>();

    println!("{}", sigil);
}
