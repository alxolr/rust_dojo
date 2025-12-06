use crate::error::Result;
pub struct Solution;

impl Solution {
    pub fn part_1(input: &str) -> Result<u64> {
        let (grid, ops) = parse_input(input);
        let mut answer = 0;

        let rows = grid.len();
        let cols = grid[0].len();

        let mut new_grid = vec![vec![0; rows]; cols];

        for row in 0..rows {
            for col in 0..cols {
                new_grid[col][row] = grid[row][col].parse().unwrap_or_default();
            }
        }

        for (id, line) in new_grid.iter().enumerate() {
            let line_result = match ops[id] {
                "*" => line.iter().fold(1, |mut acc, num| {
                    acc *= num;
                    acc
                }),
                _ => line.iter().sum(),
            };

            answer += line_result
        }

        // Reverse the rows and cols

        Ok(answer)
    }

    pub fn part_2(input: &str) -> Result<u64> {
        let (grid, ops) = parse_input(input);
        let mut answer = 0;

        let rows = grid.len();
        let cols = grid[0].len();

        let mut reverse_grid = vec![vec![""; rows]; cols];
        let mut cols_max_length = vec![0; cols];

        for row in 0..rows {
            for col in 0..cols {
                reverse_grid[col][row] = grid[row][col];
                cols_max_length[col] = cols_max_length[col].max(grid[row][col].len());
            }
        }

        // reparse the grid given the new len sizes
        let grid = parse_input_given_mask(input, &cols_max_length);

        let rows = grid.len();
        let cols = grid[0].len();

        let mut reverse_grid = vec![vec!["".to_string(); rows]; cols];

        for row in 0..rows {
            for col in 0..cols {
                reverse_grid[col][row] = grid[row][col].clone();
            }
        }

        for (id, line) in reverse_grid.iter().enumerate() {
            let line = right_to_left(line);
            let line_result = match ops[id] {
                "*" => line.iter().fold(1, |mut acc, num| {
                    acc *= num;
                    acc
                }),
                _ => line.iter().sum(),
            };

            answer += line_result
        }

        // Reverse the rows and cols

        Ok(answer)
    }
}

fn parse_input_given_mask<'a>(input: &'a str, mask: &Vec<usize>) -> Vec<Vec<String>> {
    let lines_count = input.lines().count(); // @TODO expensive optimise it in a better way

    let values = input
        .lines()
        .take(lines_count - 1)
        .map(|line| {
            let mut result = vec!["".to_string(); mask.len()];
            let mut start = 0;

            for id in 0..mask.len() {
                let end = (start + mask[id]).min(line.len());
                let potential_num = (&line[start..end]).replace(" ", "0");
                result[id] = right_pad(&potential_num, '0', mask[id]); // will handle the left_pads and right pads without the last one
                start = (start + mask[id] + 1).min(line.len()); // include the space
            }

            result
        })
        .collect();

    values
}

fn parse_input<'a>(input: &'a str) -> (Vec<Vec<&'a str>>, Vec<&'a str>) {
    // The plan is to extract the matrix of numbers and in the end extract the line of operators
    let lines_count = input.lines().count(); // @TODO expensive optimise it in a better way

    let values = input
        .lines()
        .take(lines_count - 1)
        .map(|line| line.split_ascii_whitespace().collect())
        .collect();

    let last = input
        .lines()
        .last()
        .unwrap_or_default()
        .split_ascii_whitespace()
        .collect::<Vec<_>>();

    (values, last)
}

fn right_to_left(line: &Vec<String>) -> Vec<u64> {
    // find the biggest len number that would be our cols value
    let cols = line.iter().map(|num| num.len()).max().unwrap_or(0);
    let rows = line.len();

    // transform it in a grid of chars
    let mut grid = vec![vec!['0'; rows]; cols];

    for (row, num) in line.iter().enumerate() {
        for (col, ch) in num.chars().enumerate() {
            grid[col][row] = ch;
        }
    }

    grid.iter()
        .flat_map(|line| {
            line.into_iter()
                .filter(|ch| *ch != &'0')
                .collect::<String>()
                .parse::<u64>()
        })
        .collect()
}

fn right_pad(original: &str, filler: char, length: usize) -> String {
    let mut padded_text = original.to_string();

    while padded_text.len() < length {
        padded_text.push(filler);
    }

    padded_text
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
