use day_template::{error, solution::Solution};
use std::fs;

use error::Result;

const TITLE: &str = "Day 7: Laboratories";

fn main() -> Result<()> {
    let input = load_from_file("./data/input")?;
    println!("{}", TITLE);

    let part_1_sol = Solution::part_1(&input)?;
    assert_eq!(1570, part_1_sol);

    let part_2_sol = Solution::part_2(&input)?;

    Ok(())
}

fn load_from_file(path: &str) -> Result<String> {
    fs::read_to_string(path).map_err(error::Error::FailReadFile)
}
