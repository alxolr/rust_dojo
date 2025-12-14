use itertools::Itertools;
use plotters::prelude::*;
use std::collections::{BTreeSet, HashMap, VecDeque};

use crate::error::Result;
pub struct Solution;

type Point = (u64, u64);

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Outside,
    Border,
    Inside,
}

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

    // The plan is the compress the grid, a simple algorithm of compression will be to get all cols and all rows
    // and sort them and re-assign in this case the idexes
    // min col will be the start column, max col will be the max column
    // min row and max row as well
    pub fn part_2(input: &str) -> Result<u64> {
        let points_iter = || parse_points(input);
        let points = points_iter().collect::<Vec<_>>();

        let mut min_col = 100_000;
        let mut min_row = 100_000;
        let mut max_row = 0;
        let mut max_col = 0;

        let (mut compressed_rows, mut compressed_cols) = points.iter().fold(
            (BTreeSet::new(), BTreeSet::new()),
            |(mut rows, mut cols), (row, col)| {
                min_col = min_col.min(*col);
                max_col = max_col.max(*col);
                min_row = min_row.min(*row);
                max_row = max_row.max(*row);

                rows.insert(*row);
                cols.insert(*col);

                (rows, cols)
            },
        );

        // In want to have some free area, I don't want my points to be exactly on the border of the grid
        // and adding a point before and after
        compressed_rows.insert(max_row + 1);
        compressed_rows.insert(min_row - 1);

        // Similar for columns
        compressed_cols.insert(min_col - 1);
        compressed_cols.insert(max_col + 1);

        // Now we can get the new id for our grid there will be something
        let compressed_rows_map = compressed_rows
            .iter()
            .enumerate()
            .map(|(id, row)| (*row, id))
            .collect::<HashMap<u64, usize>>();

        let compressed_cols_map = compressed_cols
            .iter()
            .enumerate()
            .map(|(id, col)| (*col, id))
            .collect::<HashMap<u64, usize>>();

        let compressed_points: Vec<_> = points_iter()
            .flat_map(|(row, col)| {
                match (compressed_rows_map.get(&row), compressed_cols_map.get(&col)) {
                    (Some(row), Some(col)) => Some((*row as u64, *col as u64)),
                    _ => None,
                }
            })
            .collect();

        let cols = compressed_cols.len() + 1;
        let rows = compressed_rows.len() + 1;

        // We mark the initial grid as everything being inside
        // This will be our cached grid where we will be testing points
        let mut compressed_grid = vec![vec![Cell::Inside; cols]; rows];

        for (p1, p2) in compressed_points.into_iter().circular_tuple_windows() {
            let (p1_row, p1_col) = p1;
            let (p2_row, p2_col) = p2;

            let min_row = p1_row.min(p2_row);
            let max_row = p1_row.max(p2_row);

            let min_col = p1_col.min(p2_col);
            let max_col = p1_col.max(p2_col);

            // fill in the border lines start horizontal lines
            for row in min_row..=max_row {
                // In this case it doesn't matter it uses min col or max coll they should be equal
                compressed_grid[row as usize][max_col as usize] = Cell::Border;
            }

            for col in min_col..=max_col {
                // In this case it doesn't matter it uses min col or max coll they should be equal
                compressed_grid[max_row as usize][col as usize] = Cell::Border;
            }
        }

        // We know that the point [0,0] is outside the grid and the polygon it's closed
        // We will do a BFS and color all the points as outside until we hit a border
        let mut queue = VecDeque::new();
        queue.push_back((0isize, 0));
        let directions = [(1isize, 0), (-1, 0), (0, -1), (0, 1)];

        while let Some((row, col)) = queue.pop_front() {
            compressed_grid[row as usize][col as usize] = Cell::Outside;

            for (dx, dy) in directions {
                let row = row + dx;
                let col = col + dy;

                if let Some(cell) = safe_get(row, col, &compressed_grid) {
                    if cell == Cell::Inside {
                        queue.push_front((row, col));
                    }
                }
            }
        }

        let mut max_area = 0;
        let mut best_rect: Option<((usize, usize), (usize, usize))> = None;

        for (id1, point) in points.iter().enumerate() {
            for id2 in id1 + 1..points.len() {
                let p1 = compressed_point_coords(point, &compressed_rows_map, &compressed_cols_map);
                let p2 = compressed_point_coords(
                    &points[id2],
                    &compressed_rows_map,
                    &compressed_cols_map,
                );
                // check if all the points in the rectangle are Inside || Border
                if check_if_points_inside(&p1, &p2, &compressed_grid) {
                    let area = rectangle_area(point, &points[id2]);
                    if area > max_area {
                        max_area = area;
                        best_rect = Some((p1, p2));
                    }
                }
            }
        }

        // Draw the grid with the best rectangle highlighted
        let _ = draw_grid(&compressed_grid, "answer.jpg", best_rect.as_ref());

        Ok(max_area as u64)
    }
}

fn compressed_point_coords(
    point: &Point,
    rows_map: &HashMap<u64, usize>,
    cols_map: &HashMap<u64, usize>,
) -> (usize, usize) {
    match (rows_map.get(&point.0), cols_map.get(&point.1)) {
        (Some(row), Some(col)) => (*row, *col),
        _ => panic!("Should work"),
    }
}

fn check_if_points_inside(p1: &(usize, usize), p2: &(usize, usize), grid: &Vec<Vec<Cell>>) -> bool {
    let (p1_row, p1_col) = p1;
    let (p2_row, p2_col) = p2;

    let min_row = *p1_row.min(p2_row) as usize;
    let max_row = *p1_row.max(p2_row) as usize;

    let min_col = *p1_col.min(p2_col) as usize;
    let max_col = *p1_col.max(p2_col) as usize;

    for row in min_row..=max_row {
        for col in min_col..=max_col {
            if grid[row][col] == Cell::Outside {
                return false;
            }
        }
    }

    return true;
}

fn safe_get(row: isize, col: isize, grid: &Vec<Vec<Cell>>) -> Option<Cell> {
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;

    if row < 0 || row >= rows || col < 0 || col >= cols {
        None
    } else {
        Some(grid[row as usize][col as usize])
    }
}

fn rectangle_area(p1: &Point, p2: &Point) -> u64 {
    (((p1.0 as i64 - p2.0 as i64).abs() + 1) * ((p1.1 as i64 - p2.1 as i64).abs() + 1)) as u64
}

fn draw_grid(
    grid: &Vec<Vec<Cell>>,
    filename: &str,
    best_rect: Option<&((usize, usize), (usize, usize))>,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let rows = grid.len();
    let cols = if rows > 0 { grid[0].len() } else { 0 };

    if rows == 0 || cols == 0 {
        return Ok(());
    }

    // Calculate cell size to fit the image
    let cell_size = (1200 / rows.max(cols)).max(5).min(50);
    let width = cols * cell_size;
    let height = rows * cell_size;

    // Create the drawing area
    let root = BitMapBackend::new(filename, (width as u32, height as u32)).into_drawing_area();
    root.fill(&WHITE)?;

    // Draw each cell with its color
    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, cell) in row.iter().enumerate() {
            let x = col_idx * cell_size;
            let y = row_idx * cell_size;

            let color = match cell {
                Cell::Inside => &YELLOW,
                Cell::Outside => &GREEN,
                Cell::Border => &RED,
            };

            root.draw(&Rectangle::new(
                [
                    (x as i32, y as i32),
                    ((x + cell_size) as i32, (y + cell_size) as i32),
                ],
                color.filled(),
            ))?;
        }
    }

    // Draw grid lines
    for row_idx in 0..=rows {
        let y = row_idx * cell_size;
        root.draw(&PathElement::new(
            vec![(0, y as i32), (width as i32, y as i32)],
            &BLACK,
        ))?;
    }
    for col_idx in 0..=cols {
        let x = col_idx * cell_size;
        root.draw(&PathElement::new(
            vec![(x as i32, 0), (x as i32, height as i32)],
            &BLACK,
        ))?;
    }

    // Draw the largest rectangle with blue borders if found
    if let Some(((p1_row, p1_col), (p2_row, p2_col))) = best_rect {
        let min_row = (*p1_row).min(*p2_row);
        let max_row = (*p1_row).max(*p2_row);
        let min_col = (*p1_col).min(*p2_col);
        let max_col = (*p1_col).max(*p2_col);

        let x1 = min_col * cell_size;
        let y1 = min_row * cell_size;
        let x2 = (max_col + 1) * cell_size;
        let y2 = (max_row + 1) * cell_size;

        // Draw thick blue border (4 lines to make it thick)
        let blue = RGBColor(0, 0, 255);
        for thickness in 0..3 {
            root.draw(&PathElement::new(
                vec![
                    ((x1 as i32 + thickness, y1 as i32 + thickness)),
                    ((x2 as i32 - thickness, y1 as i32 + thickness)),
                    ((x2 as i32 - thickness, y2 as i32 - thickness)),
                    ((x1 as i32 + thickness, y2 as i32 - thickness)),
                    ((x1 as i32 + thickness, y1 as i32 + thickness)),
                ],
                blue.stroke_width(2),
            ))?;
        }
    }

    root.present()?;
    println!("Grid visualization saved to {}", filename);

    Ok(())
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
