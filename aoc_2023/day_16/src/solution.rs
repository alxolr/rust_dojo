use std::collections::{HashMap, HashSet};

use crate::error::Result;
pub struct Solution;

type Point = (isize, isize);
type DirectedPoint<'a> = (Point, &'a Dir);

#[derive(PartialEq, Eq, Hash, Clone)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Solution {
    pub fn part_1(input: &str) -> Result<i64> {
        let grid = grid(input);

        // in this set we will store the energized points and we will return just the number of them
        let mut energized: HashSet<Point> = HashSet::new();

        // for optimization purposes to not compute again and again the already visited paths
        // once we see that we have this point in the memo then this means we already computed the path
        // for him and we don't need to cycle again
        let mut memo: HashSet<DirectedPoint> = HashSet::new();

        let mut stack: Vec<DirectedPoint> = vec![((0, 0), &Dir::Right)];

        while !stack.is_empty() {
            let (point, dir) = stack.pop().unwrap();
            energized.insert(point);

            // we need to keep the direction as well cause it may happen that once the point was going up
            // the second time it may go down, so direction is also important
            memo.insert((point, dir));

            let next_point = match dir {
                Dir::Up => (point.0 - 1, point.1),
                Dir::Down => (point.0 + 1, point.1),
                Dir::Left => (point.0, point.1 - 1),
                Dir::Right => (point.0, point.1 + 1),
            };

            if let Some(val) = grid.get(&next_point) {
                let points = match val {
                    '|' => {
                        // split the beam up and down
                        vec![(next_point, &Dir::Up), (next_point, &Dir::Down)]
                    }
                    '-' => {
                        // splits the beam left and right
                        vec![(next_point, &Dir::Left), (next_point, &Dir::Right)]
                    }
                    '\\' => {
                        let mirror_dir = match dir {
                            &Dir::Up => &Dir::Left,
                            &Dir::Down => &Dir::Right,
                            &Dir::Left => &Dir::Up,
                            &Dir::Right => &Dir::Down,
                        };

                        vec![(next_point, mirror_dir)]
                    }
                    '/' => {
                        let mirror_dir = match dir {
                            &Dir::Up => &Dir::Right,
                            &Dir::Down => &Dir::Left,
                            &Dir::Left => &Dir::Down,
                            &Dir::Right => &Dir::Up,
                        };

                        vec![(next_point, mirror_dir)]
                    }
                    _ => {
                        vec![(next_point, dir)]
                    }
                };

                points.into_iter().for_each(|dir_point| {
                    if !memo.contains(&dir_point) {
                        stack.push(dir_point);
                    }
                });
            }
        }

        Ok(energized.len() as i64)
    }

    pub fn part_2(input: &str) -> Result<i32> {
        Ok(1)
    }
}

fn grid(input: &str) -> HashMap<Point, char> {
    let mut grid = HashMap::new();

    input.lines().enumerate().for_each(|(i, l)| {
        l.trim().chars().enumerate().for_each(|(j, c)| {
            grid.insert((i as isize, j as isize), c);
        });
    });

    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> String {
        r".|...\....
        |.-.\.....
        .....|-...
        ........|.
        ..........
        .........\
        ..../.\\..
        .-.-/..|..
        .|....-|.\
        ..//.|...."
            .to_string()
    }

    #[test]
    fn test_solution_part_1_from_example() -> Result<()> {
        let expected = 46;
        let input = example();

        assert_eq!(Solution::part_1(&input)?, expected);

        Ok(())
    }

    #[test]
    fn test_solution_part_2_from_example() -> Result<()> {
        let expected = 145;
        let input = example();

        assert_eq!(Solution::part_2(&input)?, expected);

        Ok(())
    }
}
