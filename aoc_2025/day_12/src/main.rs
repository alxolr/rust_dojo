use day_template::{error, solution::Solution};
use std::{fs, time::Instant};

use error::Result;

const TITLE: &str = "Day: Title";

fn main() -> Result<()> {
    let input = load_from_file("./data/input")?;
    println!("{}", TITLE);

    let now = Instant::now();
    let part_1_sol = Solution::part_1(&input)?;
    println!("part 1 = {} ({:.2?})", part_1_sol, now.elapsed());
    Ok(())
}

fn load_from_file(path: &str) -> Result<String> {
    fs::read_to_string(path).map_err(error::Error::FailReadFile)
}
