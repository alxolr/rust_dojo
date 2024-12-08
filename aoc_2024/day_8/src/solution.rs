use core::panic;
use std::{
    collections::{HashMap, HashSet},
    iter::successors,
};

use itertools::Itertools;

use crate::error::Result;
pub struct Solution;

type Position = (usize, usize);

impl Solution {
    pub fn common<'a, F>(input: &str, apply_strategy: F) -> Result<i32>
    where
        F: Fn(
            &(usize, usize), // left point
            &(usize, usize), // right point
            &(isize, isize), // direction vector point
            &(isize, isize), // grid size (rows, cols)
        ) -> Vec<(isize, isize)>,
    {
        let grid: Vec<Vec<char>> = load_grid(input)?;
        let grid_size = (grid.len() as isize, grid[0].len() as isize);

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

        // We use a hashset to get the distinct number of positions
        let positions: HashSet<(isize, isize)> = frequency_map
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

                        // Compute the direction vector to be able to generate the points on the same lines
                        let direction_point: (isize, isize) = get_direction_point(left, right);

                        // The strategy itself depends on the part of the problem, but it will return a Vec of positions
                        apply_strategy(left, right, &direction_point, &grid_size)
                    })
            })
            .collect();

        Ok(positions.len() as i32)
    }

    pub fn part_1(input: &str) -> Result<i32> {
        Self::common(input, |left, right, direction_point, grid_size| {
            [
                // get the point above
                minus_point(&cast(left), direction_point),
                //get the point bellow
                plus_point(&cast(right), direction_point),
            ]
            .into_iter()
            // filter only the points that are in bounds
            .filter(|point| is_in_bounds(&point, grid_size).is_some())
            .collect()
        })
    }

    pub fn part_2(input: &str) -> Result<i32> {
        Self::common(input, |left, right, direction, grid_size| {
            [cast(left), cast(right)] // Add current left and right point to the positions
                .into_iter()
                .chain(successors(
                    // Super fancy way to write a while loop as iterator, the idea we loop until we are not in bounds
                    is_in_bounds(&plus_point(&cast(right), &direction), grid_size),
                    |prev| {
                        let next_point = plus_point(prev, direction);

                        is_in_bounds(&next_point, grid_size)
                    },
                ))
                .chain(successors(
                    // We do a similar way for the minus direaction
                    is_in_bounds(&minus_point(&cast(left), &direction), grid_size),
                    |prev| {
                        let next_point = minus_point(prev, direction);

                        is_in_bounds(&next_point, grid_size)
                    },
                ))
                .collect()
        })
    }
}

fn plus_point(point: &(isize, isize), direction_point: &(isize, isize)) -> (isize, isize) {
    (
        point.0 as isize + direction_point.0,
        point.1 as isize + direction_point.1,
    )
}

fn minus_point(point: &(isize, isize), direction_point: &(isize, isize)) -> (isize, isize) {
    (
        point.0 as isize - direction_point.0,
        point.1 as isize - direction_point.1,
    )
}

fn load_grid(input: &str) -> Result<Vec<Vec<char>>> {
    Ok(input.lines().map(|line| line.chars().collect()).collect())
}

fn get_direction_point(point_1: &Position, point_2: &Position) -> (isize, isize) {
    (
        point_2.0 as isize - point_1.0 as isize,
        point_2.1 as isize - point_1.1 as isize,
    )
}

fn cast(point: &(usize, usize)) -> (isize, isize) {
    (point.0 as isize, point.1 as isize)
}

fn is_in_bounds(point: &(isize, isize), grid: &(isize, isize)) -> Option<(isize, isize)> {
    let (row, col) = point;
    let (rows, cols) = grid;

    if row >= &0 && row < rows && col >= &0 && col < cols {
        Some((*row, *col))
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
