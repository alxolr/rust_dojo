use crate::error::Result;
pub struct Solution;

enum Dir {
    Row,
    Col,
}

impl Solution {
    pub fn part_1(input: &str) -> Result<i64> {
        let mut total: i64 = 0;
        let grid = grid(input);

        let n = grid.len();
        let m = grid[0].len();

        for j in 0..m {
            let line = grid
                .iter()
                .take(n)
                .map(|it| it[j])
                .rev()
                .collect::<String>();

            let slide = slide(&line);

            let counts = slide.iter().enumerate().fold(0, |mut acc, (idx, it)| {
                if it == &'O' {
                    acc += idx as i64 + 1;
                }

                acc
            });

            total += counts
        }

        Ok(total)
    }

    // vec![]

    pub fn part_2(input: &str) -> Result<i64> {
        let mut grid = grid(input);

        let rows = grid.len();
        let cols = grid[0].len();

        for cycle in 1..=3 {
            slide_north(&mut grid, rows, cols);
            slide_west(&mut grid, rows);
            slide_south(&mut grid, rows, cols);
            slide_east(&mut grid, rows);

            println!("Cycle {cycle}");
            for row in 0..rows {
                println!("{}", grid[row].iter().collect::<String>())
            }
        }

        let count = count_load(&grid);

        Ok(count as i64)
    }
}

fn slide_south(grid: &mut Vec<Vec<char>>, rows: usize, cols: usize) {
    for col in 0..cols {
        let line = grid.iter().take(rows).map(|it| it[col]).collect::<String>();

        let line = slide(&line);
        update_grid(grid, line, Dir::Col, col)
    }
}

fn slide_north(grid: &mut Vec<Vec<char>>, rows: usize, cols: usize) {
    for col in 0..cols {
        let line = grid
            .iter()
            .take(rows)
            .map(|it| it[col])
            .rev()
            .collect::<String>();

        let mut line = slide(&line);
        line.reverse();

        update_grid(grid, line, Dir::Col, col)
    }
}

fn slide_west(grid: &mut Vec<Vec<char>>, rows: usize) {
    for row in 0..rows {
        let line: String = grid[row].iter().rev().collect();
        let mut line = slide(&line);
        line.reverse();

        update_grid(grid, line, Dir::Row, row);
    }
}

fn slide_east(grid: &mut Vec<Vec<char>>, rows: usize) {
    for row in 0..rows {
        let line: String = grid[row].iter().collect();
        let line = slide(&line);

        update_grid(grid, line, Dir::Row, row);
    }
}

fn update_grid(grid: &mut Vec<Vec<char>>, line: Vec<char>, dir: Dir, idx: usize) {
    match dir {
        Dir::Row => {
            grid[idx] = line;
        }
        Dir::Col => {
            let n = grid.len();

            for i in 0..n {
                grid[i][idx] = line[i]
            }
        }
    }
}

fn count_load(grid: &Vec<Vec<char>>) -> usize {
    let grid_len = grid.len();

    grid.iter()
        .enumerate()
        .map(|(idx, it)| {
            let rocks = it.iter().filter(|x| *x == &'O').count();

            (grid_len - idx) * rocks
        })
        .sum()
}

fn grid(s: &str) -> Vec<Vec<char>> {
    s.lines().map(|s| s.trim().chars().collect()).collect()
}

fn slide(line: &str) -> Vec<char> {
    line.split("#")
        .map(slide_rocks)
        .collect::<Vec<_>>()
        .join("#")
        .chars()
        .collect()
}

// in this rock group we will have only soil and rocks nothing else
// the algorithm will count the rocks and generate a string filled with
// soil and the number of rocks
fn slide_rocks<'a>(s: &str) -> String {
    let n = s.len();

    let rocks_len = s.chars().filter(|c| c == &'O').count();
    let soil_len = n - rocks_len;

    let soil_str = ".".repeat(soil_len);
    let rock_str = "O".repeat(rocks_len);

    format!("{}{}", soil_str, rock_str)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> String {
        r#"O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#...."#
            .to_string()
    }

    #[test]
    fn test_slide_line_ok() -> Result<()> {
        let scenarios = vec![
            ("O....#....", "....O#....".chars().collect::<Vec<char>>()),
            ("O.OO#....#", ".OOO#....#".chars().collect()),
            (".....##...", ".....##...".chars().collect()),
            (".O.....O#.", "......OO#.".chars().collect()),
            (".......O..", ".........O".chars().collect()),
            ("#OO..#....", "#..OO#....".chars().collect()),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = slide(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });

        Ok(())
    }

    #[test]
    fn test_solution_part_1_from_example() -> Result<()> {
        let expected = 136;
        let input = example();

        assert_eq!(Solution::part_1(&input)?, expected);

        Ok(())
    }

    #[test]
    fn test_solution_part_2_from_example() -> Result<()> {
        let expected = 136;
        let input = example();

        assert_eq!(Solution::part_2(&input)?, expected);

        Ok(())
    }
}
