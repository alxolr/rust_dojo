use std::{
    collections::HashMap,
    ops::{Range, RangeInclusive},
};

use regex::Regex;

use crate::error::Result;
pub struct Solution;

type Position = (isize, isize);
type Velocity = (isize, isize);

pub struct Robot {
    pub position: Position,
    pub velocity: Velocity,
}

impl Solution {
    pub fn part_2(input: &str, rows: usize, cols: usize) -> Result<u64> {
        let mut robots = load_robots(input);

        // run simulation
        let mut min_sec = 0;
        for sec in 0..10000 {
            min_sec = sec;
            update_robot_positions(&mut robots, rows, cols);

            let robots_by_pos: HashMap<(isize, isize), Vec<&Robot>> =
                robots.iter().fold(HashMap::new(), |mut acc, robot| {
                    acc.entry(robot.position).or_insert(vec![]).push(robot);

                    acc
                });

            let mut max_adjacent_count = 0;
            for row in 0..rows {
                let mut adjacent_robot_count = 0;
                for col in 0..cols {
                    if let Some(_) = robots_by_pos.get(&(row as isize, col as isize)) {
                        adjacent_robot_count += 1;
                        max_adjacent_count = max_adjacent_count.max(adjacent_robot_count);
                    } else {
                        adjacent_robot_count = 0;
                    }
                }
            }

            if max_adjacent_count >= 20 {
                break;
            }
        }

        Ok(min_sec + 1 as u64)
    }

    pub fn part_1(input: &str, rows: usize, cols: usize) -> Result<u64> {
        let mut robots = load_robots(input);

        // run simulation
        for _ in 0..100 {
            update_robot_positions(&mut robots, rows, cols);
        }

        let robots_by_pos = robots.iter().fold(HashMap::new(), |mut acc, robot| {
            acc.entry(robot.position).or_insert(vec![]).push(robot);

            acc
        });

        let safety_factor = [
            count_quadrant(&robots_by_pos, 0..(rows - 1) / 2, 0..(cols - 1) / 2),
            count_quadrant(&robots_by_pos, 0..rows / 2, cols / 2 + 1..cols),
            count_quadrant(&robots_by_pos, rows / 2 + 1..rows, 0..cols / 2),
            count_quadrant(&robots_by_pos, rows / 2 + 1..rows, cols / 2 + 1..cols),
        ]
        .iter()
        .filter(|count| count > &&0)
        .fold(1, |mut acc, item| {
            acc *= item;

            acc
        });

        Ok(safety_factor as u64)
    }
}

fn update_robot_positions(robots: &mut Vec<Robot>, rows: usize, cols: usize) {
    for robot in robots.iter_mut() {
        let (pos_x, pos_y) = robot.position;
        let (vel_x, vel_y) = robot.velocity;

        let mut new_pos_x = pos_x + vel_x;
        let mut new_pos_y = pos_y + vel_y;

        if new_pos_x < 0 {
            new_pos_x = rows as isize + new_pos_x;
        }

        if new_pos_x >= rows as isize {
            new_pos_x = new_pos_x - rows as isize;
        }

        if new_pos_y < 0 {
            new_pos_y = cols as isize + new_pos_y;
        }

        if new_pos_y >= cols as isize {
            new_pos_y = new_pos_y - cols as isize;
        }

        robot.position = (new_pos_x, new_pos_y);
    }
}

fn count_quadrant(
    robots: &HashMap<(isize, isize), Vec<&Robot>>,
    rows_range: Range<usize>,
    cols_range: Range<usize>,
) -> u64 {
    let mut count = 0;
    for row in rows_range {
        for col in cols_range.clone() {
            if let Some(robots) = robots.get(&(row as isize, col as isize)) {
                count += robots.len();
            }
        }
    }

    count as u64
}

fn load_robots(input: &str) -> Vec<Robot> {
    input
        .lines()
        .map(|line| {
            let pattern = Regex::new(r#"p=(?<px>\d+),(?<py>\d+)\sv=(?<vx>-*\d+),(?<vy>-*\d+)"#)
                .expect("Invalid Regex");

            if let Some(captures) = pattern.captures(line) {
                let px = captures["px"].parse::<isize>().unwrap();
                let py = captures["py"].parse::<isize>().unwrap();
                let vx = captures["vx"].parse::<isize>().unwrap();
                let vy = captures["vy"].parse::<isize>().unwrap();

                Robot {
                    position: (py, px),
                    velocity: (vy, vx), // not sure they do it reversed
                }
            } else {
                panic!("Invalid line structure");
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() -> Result<()> {
        let input = r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"#;
        assert_eq!(Solution::part_1(input, 7, 11)?, 12);

        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let input = r#""#;
        assert_eq!(Solution::part_2(input, 7, 11)?, 10000);

        Ok(())
    }
}
