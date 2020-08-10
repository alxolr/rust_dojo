extern crate linked_hash_set;
use linked_hash_set::LinkedHashSet;

fn main() {
    let intent = std::env::args().nth(1).expect("no pattern given");
    let vovewls = vec!['A', 'E', 'I', 'O', 'U']
        .into_iter()
        .collect::<LinkedHashSet<char>>();

    let sigil = intent
        .to_uppercase()
        .chars()
        .filter(|c| !vovewls.contains(c))
        .collect::<LinkedHashSet<char>>()
        .into_iter()
        .collect::<String>();

    println!("{}", sigil);
}
