use std::collections::{HashMap, HashSet};

use crate::error::Result;
pub struct Solution;

type Point = (isize, isize);
type DirectedPoint<'a> = (Point, &'a Dir);

pub struct Grid {
    data: HashMap<Point, char>,
    rows: isize,
    cols: isize,
}

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
        let mut stack: Vec<DirectedPoint> = vec![((0, -1), &Dir::Right)];

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

            if let Some(val) = grid.data.get(&next_point) {
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

        Ok(energized.len() as i64 - 1)
    }

    pub fn part_2(input: &str) -> Result<i64> {
        let grid = grid(input);
        let mut max_energized = 0;
        let left_right = (0..grid.rows)
            .into_iter()
            .map(|row: isize| vec![((row, -1), &Dir::Right), ((row, grid.cols), &Dir::Left)])
            .flatten();

        let top_down = (0..grid.cols)
            .into_iter()
            .map(|col: isize| vec![((-1, col), &Dir::Down), ((grid.rows, col), &Dir::Up)])
            .flatten();

        let starters: Vec<DirectedPoint> = left_right.chain(top_down).collect();

        for starter in starters {
            let mut stack = vec![starter];

            // in this set we will store the energized points and we will return just the number of them
            let mut energized: HashSet<Point> = HashSet::new();

            // for optimization purposes to not compute again and again the already visited paths
            // once we see that we have this point in the memo then this means we already computed the path
            // for him and we don't need to cycle again
            let mut memo: HashSet<DirectedPoint> = HashSet::new();

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

                if let Some(val) = grid.data.get(&next_point) {
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

            max_energized = max_energized.max(energized.len() - 1);
        }

        Ok(max_energized as i64)
    }
}

fn grid(input: &str) -> Grid {
    let mut grid = HashMap::new();

    let mut rows = 0;
    let mut cols = 0;

    input.lines().enumerate().for_each(|(i, l)| {
        rows = i;
        l.trim().chars().enumerate().for_each(|(j, c)| {
            cols = j;
            grid.insert((i as isize, j as isize), c);
        });
    });

    Grid {
        data: grid,
        rows: rows as isize + 1,
        cols: cols as isize + 1,
    }
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
        let expected = 51;
        let input = example();

        assert_eq!(Solution::part_2(&input)?, expected);

        Ok(())
    }
}
