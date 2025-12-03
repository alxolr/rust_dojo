mod error;
mod solution;

use std::{fs, time::Instant};

use error::Result;
use solution::Solution;

const TITLE: &str = "Day 3: Lobby";

fn main() -> Result<()> {
    let input = load_from_file("./data/input")?;
    println!("{}", TITLE);

    let now = Instant::now();
    let part_1_sol = Solution::part_1(&input)?;
    assert_eq!(part_1_sol, 17085);
    println!("part 1 = {} ({:.2?})", part_1_sol, now.elapsed());

    let now = Instant::now();
    let part_2_sol = Solution::part_2(&input)?;
    assert_eq!(part_2_sol, 169408143086082);
    println!("part 2 = {} ({:.2?})", part_2_sol, now.elapsed());

    Ok(())
}

fn load_from_file(path: &str) -> Result<String> {
    fs::read_to_string(path).map_err(error::Error::FailReadFile)
}
