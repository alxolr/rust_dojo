mod error;
mod solution;

use std::{fs, time::Instant};

use error::Result;
use solution::Solution;

const TITLE: &str = "Day 4: Printing Department";

fn main() -> Result<()> {
    let input = load_from_file("./data/input")?;
    println!("{}", TITLE);

    let now = Instant::now();
    let part_1_sol = Solution::part_1(&input)?;
    println!("part 1 = {} ({:.2?})", part_1_sol, now.elapsed());
    assert_eq!(1433, part_1_sol);

    let now = Instant::now();
    let part_2_sol = Solution::part_2(&input)?;
    assert_eq!(8616, part_2_sol);
    println!("part 2 = {} ({:.2?})", part_2_sol, now.elapsed());

    Ok(())
}

fn load_from_file(path: &str) -> Result<String> {
    fs::read_to_string(path).map_err(error::Error::FailReadFile)
}
