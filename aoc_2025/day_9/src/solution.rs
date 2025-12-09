use std::collections::{BTreeMap, BTreeSet, HashSet};

use crate::error::Result;
pub struct Solution;

type Point = (u64, u64);

impl Solution {
    pub fn part_1(input: &str) -> Result<u64> {
        let points = parse_points(input).collect::<Vec<_>>();
        let len = points.len();

        // Simple idea we enumerate all possible squares and find the max area
        let mut max_area = 0;
        for (id1, point) in points.iter().enumerate() {
            for id2 in id1 + 1..len {
                max_area = max_area.max(rectangle_area(point, &points[id2]));
            }
        }

        Ok(max_area as u64)
    }

    // Using the BTreeMap and BTreeSet we get the coordinates of all horizontal and vertical lines  not sure about BTreeMap
    // I think works just fine HashMap, cause we don't need to specifically check the lines in order
    // We can check using horizontal lines if our point interesects it and how many lines does it interesect
    // We will emit a ray up -y and count how many lines does it intersect and emit a ray down count how many lines does it intersect
    // We will emit left and right  // for all cases we should odd numbers to consider a point inside.
    // now we will go with points again making the rectangle, find compute the bottom left, compute the top right
    // apply the ray casting algorithm on the lines if this two points are inside then we have a valid rectangle
    pub fn part_2(input: &str) -> Result<u64> {
        let points_iter = || parse_points(input);
        let points = points_iter().collect::<Vec<_>>();
        let points_len = points.len();

        let mut max_area = 0;

        // Store the graph of connections to be able to draw the border X's
        let mut horizontal_lines = BTreeMap::new();
        let mut vertical_lines = BTreeMap::new();

        for (row, col) in points.iter() {
            let entry = horizontal_lines.entry(*row).or_insert(BTreeSet::new());
            entry.insert(*col);

            let entry = vertical_lines.entry(*col).or_insert(BTreeSet::new());
            entry.insert(*row);
        }

        for (id1, point) in points.iter().enumerate() {
            for id2 in id1 + 1..points_len {
                let other_two = get_other_two_points(point, &points[id2]);

                if other_two
                    .iter()
                    .all(|point| is_inside_polygon(point, &vertical_lines, &horizontal_lines))
                {
                    max_area = max_area.max(rectangle_area(point, &points[id2]));
                }
            }
        }

        Ok(max_area)
    }
}

fn is_inside_polygon(
    point: &Point,
    vertical_lines: &BTreeMap<u64, BTreeSet<u64>>,
    horizontal_lines: &BTreeMap<u64, BTreeSet<u64>>,
) -> bool {
    let (row, col) = point;

    let mut vertical_intersections = 0;

    // Check VERTICAL edges to the right of our point
    for (vcol, rows_set) in vertical_lines.iter() {
        if vcol <= col {
            continue;
        }

        let rows: Vec<_> = rows_set.iter().copied().collect();

        // Count how many times the ray crosses vertical segments at this column
        for window in rows.windows(2) {
            let row1 = window[0];
            let row2 = window[1];

            // Standard ray casting: count if ray passes through the segment
            // Use consistent boundary rule: count if row is strictly between endpoints
            // or if row equals the lower endpoint (to avoid double-counting at vertices)
            if *row >= row1 && *row < row2 {
                vertical_intersections += 1;
                break; // Only count once per vertical line
            }
        }
    }

    let mut horizontal_intersections = 0;

    // Check horizontal intersections bellow our point
    for (vrow, rows_set) in horizontal_lines.iter() {
        if vrow <= row {
            continue;
        }

        let cols: Vec<_> = rows_set.iter().copied().collect();

        // Count how many times the ray crosses vertical segments at this column
        for window in cols.windows(2) {
            let col1: u64 = window[0];
            let col2 = window[1];

            if *col >= col1 && *col < col2 {
                horizontal_intersections += 1;
                break; // Only count once per horizontal line
            }
        }
    }

    vertical_intersections % 2 == 1 && horizontal_intersections % 2 == 1
}

fn get_other_two_points(point1: &Point, point2: &Point) -> [Point; 2] {
    // this means point1 is top_right
    let top_left = (point1.0, point2.1);
    let bottom_right = (point2.0, point1.1);

    [top_left, bottom_right]
}

fn rectangle_area(p1: &Point, p2: &Point) -> u64 {
    (((p1.0 as i64 - p2.0 as i64).abs() + 1) * ((p1.1 as i64 - p2.1 as i64).abs() + 1)) as u64
}

fn parse_points(input: &str) -> impl Iterator<Item = Point> + use<'_> {
    input.lines().map(|line| {
        let mut parts = line.trim().split(",");
        let col = parts.next().unwrap_or_default().parse::<u64>().unwrap_or(0);
        let row = parts.next().unwrap_or_default().parse::<u64>().unwrap_or(0);

        (row, col)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() -> Result<()> {
        let input = r#"7,1
        11,1
        11,7
        9,7
        9,5
        2,5
        2,3
        7,3"#;
        assert_eq!(Solution::part_1(input)?, 50);

        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let input = r#"7,1
        11,1
        11,7
        9,7
        9,5
        2,5
        2,3
        7,3"#;
        assert_eq!(Solution::part_2(input)?, 24);

        Ok(())
    }
}
