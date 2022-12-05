use std::fs::read_to_string;

fn main() {
    let data = read_to_string("./input.data").unwrap();

    let mut results = data
        .split("\n\n")
        .map(|chunk| {
            chunk
                .lines()
                .map(|line| line.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<_>>();

    results.sort();

    let result:u32 = results.iter().rev().take(3).sum();

    println!("{}", result);
}
