use ordered_float::OrderedFloat;
use std::collections::BinaryHeap;

use crate::{error::Result, uf::UnionFind};
pub struct Solution;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point([OrderedFloat<f64>; 3]);

#[derive(Debug, PartialEq, Eq)]
struct Fuse<'a>(OrderedFloat<f64>, &'a Point, &'a Point);

impl<'a> Ord for Fuse<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Reverse ordering for descending sort (max heap behavior)
        other
            .0
            .partial_cmp(&self.0)
            .unwrap_or(std::cmp::Ordering::Equal)
    }
}

impl<'a> PartialOrd for Fuse<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn part_1(input: &str, max_junctions: usize) -> Result<u64> {
        let points = parse_input(input)?;
        let points_count = points.len();

        // Calculate the distances and add in a binary heap
        let mut binary_heap = BinaryHeap::with_capacity((points_count * (points_count + 1)) / 2);
        for p1 in 0..points.len() {
            for p2 in p1 + 1..points.len() {
                let distance = distance(&points[p1], &points[p2]);
                let fuse = Fuse(distance, &points[p1], &points[p2]);
                binary_heap.push(fuse);
            }
        }

        // Initialize UnionFind
        let mut uf = UnionFind::new(points_count);
        let mut junctions = 0;

        // Process fuses in descending order of distance
        while let Some(fuse) = binary_heap.pop() {
            junctions += 1;
            let Fuse(_, p1, p2) = fuse;

            // Find indices of the points
            let idx1 = points.iter().position(|p| p == p1).unwrap_or_default();
            let idx2 = points.iter().position(|p| p == p2).unwrap_or_default();

            // Union the two points
            uf.union(idx1, idx2);

            if junctions == max_junctions {
                break;
            }
        }

        let sizes = uf.get_component_sizes();
        let mut junctions = sizes.values().collect::<Vec<_>>();
        junctions.sort_unstable();

        let response = junctions.iter().rev().take(3).fold(1, |mut acc, it| {
            acc *= *it;
            acc
        });

        Ok(response as u64)
    }

    pub fn part_2(input: &str) -> Result<f64> {
        let points = parse_input(input)?;
        let points_count = points.len();

        // Calculate the distances and add in a binary heap
        let mut binary_heap = BinaryHeap::with_capacity((points_count * (points_count + 1)) / 2);
        for p1 in 0..points.len() {
            for p2 in p1 + 1..points.len() {
                let distance = distance(&points[p1], &points[p2]);
                let fuse = Fuse(distance, &points[p1], &points[p2]);
                binary_heap.push(fuse);
            }
        }

        // Initialize UnionFind
        let mut uf = UnionFind::new(points_count);

        // Process fuses in descending order of distance
        while let Some(fuse) = binary_heap.pop() {
            let Fuse(_, p1, p2) = fuse;

            // Find indices of the points
            let idx1 = points.iter().position(|p| p == p1).unwrap_or_default();
            let idx2 = points.iter().position(|p| p == p2).unwrap_or_default();

            // Union the two points
            uf.union(idx1, idx2);

            if uf.count_components() == 1 {
                let OrderedFloat(result) = p1.0[0] * p2.0[0];

                return Ok(result);
            }
        }

        panic!("Should've returned already")
    }
}

fn distance(point1: &Point, point2: &Point) -> OrderedFloat<f64> {
    let Point([x1, y1, z1]) = point1;
    let Point([x2, y2, z2]) = point2;

    let dx = (x2 - x1).powf(2.0);
    let dy = (y2 - y1).powf(2.0);
    let dz = (z2 - z1).powf(2.0);

    let distance = (dx + dy + dz).sqrt();

    OrderedFloat(distance)
}

fn parse_input(input: &str) -> Result<Vec<Point>> {
    let points = input
        .lines()
        .map(|line| {
            let [x, y, z] = line
                .trim()
                .split(",")
                .flat_map(|val| val.parse::<f64>())
                .collect::<Vec<_>>()[..]
            else {
                panic!("Broken line");
            };

            Point([OrderedFloat(x), OrderedFloat(y), OrderedFloat(z)])
        })
        .collect();

    Ok(points)
}

#[cfg(test)]
mod tests {
    use super::*;

    const COMMON_INPUT: &str = r#"162,817,812
    57,618,57
    906,360,560
    592,479,940
    352,342,300
    466,668,158
    542,29,236
    431,825,988
    739,650,466
    52,470,668
    216,146,977
    819,987,18
    117,168,530
    805,96,715
    346,949,466
    970,615,88
    941,993,340
    862,61,35
    984,92,344
    425,690,689"#;

    #[test]
    fn test_part_1() -> Result<()> {
        assert_eq!(Solution::part_1(COMMON_INPUT, 10)?, 40);

        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        assert_eq!(Solution::part_2(COMMON_INPUT)?, 25272.0);

        Ok(())
    }
}
