use std::{collections::VecDeque, mem::take};

use crate::error::Result;
pub struct Solution;

impl Solution {
    pub fn part_1(input: &str) -> Result<u64> {
        let (grid, operations) = parse_input_part_1(input);

        let transposed_grid = transpose(grid);
        let answer = calculate(transposed_grid, operations);

        Ok(answer)
    }

    pub fn part_2(input: &str) -> Result<u64> {
        // For part 2 the grid is just a Vec<Vec<char>>
        let (grid, operations) = parse_input_part_2(input);

        let rows = grid.len();
        let cols = grid[0].len();

        // In groups we store the list of numbers after we do all the parsing
        // I use VecDeque as I am processing in revese order, easier to push in front
        // Given the test case example in sol_1 or sol_2 groups will be:
        // [["4", "431", "623"], [..]]
        let mut groups = VecDeque::with_capacity(rows);

        // The current group is ["4", "431", "621"]
        let mut current_group = Vec::with_capacity(rows);

        // We iterate from cols in reverse order
        for col in (0..cols).rev() {
            // The current item will Be ['4'] or ['4','3','1']
            let mut current_group_item = Vec::with_capacity(rows);

            // We move from top down and try to fill the current_group_item with digits
            for (row, _) in grid.iter().take(rows).enumerate() {
                if grid[row][col] != ' ' {
                    current_group_item.push(grid[row][col])
                }
            }

            // We collect the item as string
            let group_item = current_group_item.iter().collect::<String>();

            if !group_item.is_empty() {
                // We are still in the current group iteration, and we get the current item
                current_group.push(group_item)
            } else {
                // This means the whole column is empty space so it's a delimiter
                // Take removes the value and empties the vec
                groups.push_front(take(&mut current_group));
            }
        }

        // To not forget to get the last group after iterations
        if !current_group.is_empty() {
            groups.push_front(current_group);
        }

        let answer = calculate(groups, operations);

        Ok(answer)
    }
}

fn calculate<I, T>(grid: I, operations: Vec<&str>) -> u64
where
    I: IntoIterator<Item = Vec<T>>,
    T: AsRef<str>,
{
    grid.into_iter()
        .zip(operations.iter())
        .map(|(line, operation)| {
            let line = line.iter().flat_map(|num| num.as_ref().parse::<u64>());

            match operation {
                &"*" => line.fold(1, |mut acc, num| {
                    acc *= num;
                    acc
                }),
                _ => line.sum::<u64>(),
            }
        })
        .sum::<u64>()
}

fn transpose(grid: Vec<Vec<&str>>) -> Vec<Vec<&str>> {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut rotated_grid = vec![vec![""; rows]; cols];
    for (row_id, row) in grid.iter().take(rows).enumerate() {
        for (col_id, item) in row.iter().take(cols).enumerate() {
            rotated_grid[col_id][row_id] = item;
        }
    }

    rotated_grid
}

// For part one is enough for us to just split_ascii_whitespaces and parse them as numbers after
fn parse_input_part_1(input: &str) -> (Vec<Vec<&str>>, Vec<&str>) {
    // The plan is to extract the matrix of numbers and in the end extract the line of operators
    let lines_count = input.lines().count();

    let values = input
        .lines()
        .take(lines_count - 1)
        .map(|line| line.split_ascii_whitespace().collect())
        .collect();

    let operators = input
        .lines()
        .last()
        .unwrap_or_default()
        .split_ascii_whitespace()
        .collect::<Vec<_>>();

    (values, operators)
}

fn parse_input_part_2(input: &str) -> (Vec<Vec<char>>, Vec<&str>) {
    let lines_count = input.lines().count(); // only 5 lines super cheap

    // We parse the grid as grid of chars
    let grid = input
        .lines()
        .take(lines_count - 1)
        .map(|line| line.chars().collect())
        .collect();

    let operators = input
        .lines()
        .last()
        .unwrap_or_default()
        .split_ascii_whitespace()
        .collect::<Vec<_>>();

    (grid, operators)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() -> Result<()> {
        let input = r#"123 328  51 64
         45 64  387 23
          6 98  215 314
        *   +   *   + "#;
        assert_eq!(Solution::part_1(input)?, 4277556);

        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let input = r#"123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  "#;
        assert_eq!(Solution::part_2(input)?, 3263827);

        Ok(())
    }
}
