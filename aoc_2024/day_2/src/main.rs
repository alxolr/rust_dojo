mod error;
mod solution;

use std::{fs, time::Instant};

use error::Result;
use solution::Solution;

const TITLE: &'static str = "Day 2: Red-Nosed Reports";

fn main() -> Result<()> {
    let input = load_from_file("./data/input")?;
    println!("{}", TITLE);

    let now = Instant::now();
    let part_1_sol = Solution::part_1(&input)?;
    println!("  part 1 = {} ({:.2?})", part_1_sol, now.elapsed());

    assert_eq!(part_1_sol, 246); // Used after refactoring to check the result

    let now = Instant::now();
    let part_2_sol = Solution::part_2(&input)?;
    println!("  part 2 = {} ({:.2?})", part_2_sol, now.elapsed());

    assert_eq!(part_2_sol, 318);

    Ok(())
}

fn load_from_file(path: &str) -> Result<String> {
    fs::read_to_string(path).map_err(|e| error::Error::FailReadFile(e))
}
