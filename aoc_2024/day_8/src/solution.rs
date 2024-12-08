use core::panic;
use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::error::Result;
pub struct Solution;

type Position = (usize, usize);

impl Solution {
    pub fn part_1(input: &str) -> Result<i32> {
        let grid: Vec<Vec<char>> = load_grid(input)?;

        // Extract a map of all antenas, group them by frequency and store their positions
        let frequency_map = grid
            .iter()
            .enumerate()
            .fold(HashMap::new(), |mut acc, (row, line)| {
                line.iter()
                    .enumerate()
                    .filter(|(_, ch)| *ch != &'.')
                    .for_each(|(col, ch)| {
                        acc.entry(ch).or_insert(vec![]).push((row, col));
                    });
                acc
            });

        let locations: HashSet<(isize, isize)> = frequency_map
            .values()
            .flat_map(|positions| {
                positions
                    .iter()
                    // This is a fancy way to do all combinations of pairs of antenas
                    // an iterator alternative to do two for loops
                    .combinations(2)
                    .flat_map(|pair| {
                        let [left, right] = pair[..] else {
                            panic!("wrong definition")
                        };
                        // Compute the antinodes there should be only two for each pair
                        // Step one I guess we need to calculate the distance between the pair
                        let direction_vector = get_direction_vector(left, right);
                        [
                            minus_point(left, &direction_vector),
                            plus_point(right, &direction_vector),
                        ]
                        .into_iter()
                        .filter(|point| is_bound(&point, &grid).is_some())
                    })
            })
            .collect();

        Ok(locations.len() as i32)
    }

    pub fn part_2(input: &str) -> Result<i32> {
        let grid: Vec<Vec<char>> = load_grid(input)?;

        // Extract a map of all antenas, group them by frequency and store their positions
        let frequency_map = grid
            .iter()
            .enumerate()
            .fold(HashMap::new(), |mut acc, (row, line)| {
                line.iter()
                    .enumerate()
                    .filter(|(_, ch)| *ch != &'.')
                    .for_each(|(col, ch)| {
                        acc.entry(ch).or_insert(vec![]).push((row, col));
                    });
                acc
            });

        let mut locations = HashSet::<(isize, isize)>::new();

        frequency_map.values().for_each(|positions| {
            positions
                .iter()
                // This is a fancy way to do all combinations of pairs of antenas
                // an iterator alternative to do two for loops
                .combinations(2)
                .for_each(|pair| {
                    let [left, right] = pair[..] else {
                        panic!("wrong definition")
                    };

                    locations.insert((left.0 as isize, left.1 as isize));
                    locations.insert((right.0 as isize, right.1 as isize));

                    // The starting point is to calcualte the direction vector
                    let direction_point: (isize, isize) = get_direction_vector(left, right);

                    let mut plus_antinodes = vec![];
                    let mut plus_antinode: (isize, isize) = plus_point(right, &direction_point);

                    while is_bound(&plus_antinode, &grid).is_some() {
                        plus_antinodes.push(plus_antinode);
                        plus_antinode = plus_point(
                            &(plus_antinode.0 as usize, plus_antinode.1 as usize),
                            &direction_point,
                        );
                    }

                    let mut minus_antinodes = vec![];
                    let mut minus_antinode: (isize, isize) = minus_point(left, &direction_point);

                    while is_bound(&minus_antinode, &grid).is_some() {
                        minus_antinodes.push(minus_antinode);
                        minus_antinode = minus_point(
                            &(minus_antinode.0 as usize, minus_antinode.1 as usize),
                            &direction_point,
                        );
                    }

                    plus_antinodes
                        .iter()
                        .chain(minus_antinodes.iter())
                        .filter(|point| is_bound(&point, &grid).is_some())
                        .for_each(|pos| {
                            locations.insert(*pos);
                        });
                });
        });

        Ok(locations.len() as i32)
    }
}

fn plus_point(point: &Position, direction_point: &(isize, isize)) -> (isize, isize) {
    (
        point.0 as isize + direction_point.0,
        point.1 as isize + direction_point.1,
    )
}

fn minus_point(point: &Position, direction_point: &(isize, isize)) -> (isize, isize) {
    (
        point.0 as isize - direction_point.0,
        point.1 as isize - direction_point.1,
    )
}

fn load_grid(input: &str) -> Result<Vec<Vec<char>>> {
    Ok(input.lines().map(|line| line.chars().collect()).collect())
}

fn get_direction_vector(point_1: &Position, point_2: &Position) -> (isize, isize) {
    (
        point_2.0 as isize - point_1.0 as isize,
        point_2.1 as isize - point_1.1 as isize,
    )
}

fn is_bound<'a>(point: &'a (isize, isize), grid: &[Vec<char>]) -> Option<&'a (isize, isize)> {
    let (row, col) = point;

    if row >= &0 && row < &(grid.len() as isize) && col >= &0 && col < &(grid[0].len() as isize) {
        Some(point)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() -> Result<()> {
        let input: &str = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;
        assert_eq!(Solution::part_1(input)?, 14);

        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let input = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;
        assert_eq!(Solution::part_2(input)?, 34);

        Ok(())
    }
}
