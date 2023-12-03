#![allow(dead_code)]

struct Solution;

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

impl Solution {
    // region:      --- Part Two
    fn part_2(lines: &Vec<String>) -> Result<u32> {
        let result = lines
            .iter()
            .enumerate()
            .flat_map(|(idx, line)| {
                let top = if idx == 0 {
                    None
                } else {
                    Some(lines.get(idx - 1).unwrap().as_str())
                };

                let bottom = if idx == lines.len() - 1 {
                    None
                } else {
                    Some(lines.get(idx + 1).unwrap().as_str())
                };

                let mut gear_pairs = vec![];
                for (pos, ch) in line.chars().enumerate() {
                    if ch == '*' {
                        gear_pairs.push(Solution::find_gear_pair(pos, line, top, bottom).unwrap())
                    }
                }

                gear_pairs.iter().filter_map(|x| *x).collect::<Vec<_>>()
            })
            .fold(0, |mut acc, pair| {
                acc += pair.0 * pair.1;
                acc
            });

        Ok(result)
    }

    fn find_gear_pair(
        pos: usize,
        line: &str,
        top_line: Option<&str>,
        bot_line: Option<&str>,
    ) -> Result<Option<(u32, u32)>> {
        let n = line.len();

        //  top_left.[direct_adjacent].top_right
        //              left*right
        //  bot left.[direct_adjacent].bot right

        let direct_right = |line: &str, start: usize, end: usize| -> Option<u32> {
            let maybe_number = line[start + 1..=end]
                .chars()
                .take_while(|x| x.is_ascii_digit())
                .collect::<String>();

            if let Ok(number) = maybe_number.parse::<u32>() {
                Some(number)
            } else {
                None
            }
        };

        let direct_left = |line: &str, start: usize, end: usize| -> Option<u32> {
            let maybe_number = line[start..end]
                .chars()
                .rev()
                .take_while(|x| x.is_ascii_digit())
                .collect::<Vec<_>>()
                .iter()
                .rev()
                .collect::<String>();

            if let Ok(number) = maybe_number.parse::<u32>() {
                Some(number)
            } else {
                None
            }
        };

        let direct_adjacent = |pos: usize, line: Option<&str>| -> Option<u32> {
            if let Some(line) = line {
                let line_chs: Vec<char> = line.chars().collect();
                // if it's above
                let mut skewed_pos = pos;

                if line_chs[pos].is_ascii_digit() {
                    // we need to skew the position to the right to include entire number

                    while line_chs[skewed_pos].is_ascii_digit() {
                        skewed_pos += 1
                    }

                    direct_left(line, 0, skewed_pos)
                } else {
                    None
                }
            } else {
                None
            }
        };

        let diagonal_left_finder = |pos: usize, line: Option<&str>| -> Option<u32> {
            if let Some(line) = line {
                // the assumption that it's on the left diagonal
                let line_chs: Vec<char> = line.chars().collect();

                if !line_chs[pos].is_ascii_digit() {
                    direct_left(line, 0, pos)
                } else {
                    None
                }
            } else {
                None
            }
        };

        let diagonal_right_finder = |pos, size: isize, line: Option<&str>| -> Option<u32> {
            if let Some(line) = line {
                let line_chs: Vec<char> = line.chars().collect();

                if !line_chs[pos as usize].is_ascii_digit() {
                    direct_right(line, pos as usize, (size - 1) as usize)
                } else {
                    None
                }
            } else {
                None
            }
        };

        let adjacent_numbers = [
            diagonal_left_finder(pos, top_line),
            direct_adjacent(pos, top_line),
            diagonal_right_finder(pos as isize, n as isize, top_line),
            direct_left(line, 0, pos),
            direct_right(line, pos, n - 1),
            diagonal_left_finder(pos, bot_line),
            direct_adjacent(pos, bot_line),
            diagonal_right_finder(pos as isize, n as isize, bot_line),
        ]
        .iter()
        .filter_map(|x| *x)
        .collect::<Vec<u32>>();

        if adjacent_numbers.len() == 2 {
            Ok(Some((adjacent_numbers[0], adjacent_numbers[1])))
        } else {
            Ok(None)
        }
    }

    // endregion:   --- Part Two

    // region:      --- Part One
    fn part_1(lines: &Vec<String>) -> Result<u32> {
        let result = lines
            .iter()
            .enumerate()
            .flat_map(|(idx, line)| {
                let top = if idx == 0 {
                    None
                } else {
                    Some(lines.get(idx - 1).unwrap().as_str())
                };

                let bottom = if idx == lines.len() - 1 {
                    None
                } else {
                    Some(lines.get(idx + 1).unwrap().as_str())
                };

                Solution::process_part_1_line(line, top, bottom).unwrap()
            })
            .sum::<u32>();

        Ok(result)
    }

    fn process_part_1_line(
        line: &str,
        top: Option<&str>,
        bottom: Option<&str>,
    ) -> Result<Vec<u32>> {
        let vec_line: Vec<char> = line.chars().collect();
        let n = vec_line.len();

        let mut stack = String::new();
        let mut result = vec![];
        let mut is_valid = false;

        for (ch_idx, ch) in vec_line.iter().enumerate() {
            match ch {
                ch if ch.is_ascii_digit() => {
                    stack.push(*ch);

                    is_valid =
                        is_valid || Solution::is_adjacent_to_symbol(ch_idx, n, line, top, bottom);
                }
                _ => {
                    if !stack.is_empty() {
                        let val = stack.parse::<u32>().unwrap();

                        if is_valid {
                            result.push(val);
                        }
                        stack = String::new();
                        is_valid = false;
                    }
                }
            }
        }

        if !stack.is_empty() {
            let val = stack.parse::<u32>().unwrap();

            if is_valid {
                result.push(val);
            }
        }

        Ok(result)
    }

    fn is_adjacent_to_symbol(
        ch_idx: usize,
        n: usize,
        current_line: &str,
        maybe_top: Option<&str>,
        maybe_bottom: Option<&str>,
    ) -> bool {
        let start = if ch_idx > 0 { ch_idx - 1 } else { 0 };
        let end = if ch_idx < n - 1 { ch_idx + 1 } else { ch_idx };

        let is_valid = |maybe_line: Option<&str>| -> bool {
            if let Some(maybe_line) = maybe_line {
                let maybe_line: Vec<char> = maybe_line.chars().collect();

                let output = maybe_line[start..=end]
                    .iter()
                    .any(|x| !x.is_ascii_digit() && *x != '.');

                output
            } else {
                false
            }
        };

        is_valid(maybe_top) || is_valid(maybe_bottom) || is_valid(Some(current_line))
    }

    // endregion:   --- Part one
}

#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };

    use super::*;

    fn example_engine() -> Vec<String> {
        vec![
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect()
    }

    #[test]
    fn test_part_2_file_ok() -> Result<()> {
        let contents = load_contents()?;

        let result = Solution::part_2(&contents)?;
        assert_eq!(result, 84363105);

        Ok(())
    }

    #[test]
    fn test_part_2_example_ok() -> Result<()> {
        assert_eq!(Solution::part_2(&example_engine())?, 467835);

        Ok(())
    }

    #[test]
    fn test_find_gear_pairs_part_two_ok() -> Result<()> {
        let top = Some("123..");
        let line = "..*..";
        let bot = Some("...10");
        let gear_position = line.chars().position(|x| x == '*').unwrap();

        let result = Solution::find_gear_pair(gear_position, line, top, bot)?;

        assert_eq!(result, Some((123, 10)));

        let line = "..10*11..";
        let gear_position = line.chars().position(|x| x == '*').unwrap();
        let result = Solution::find_gear_pair(gear_position, line, None, None)?;

        if let Some(pair) = result {
            assert_eq!(pair, (10, 11))
        };

        let top = Some("123...");
        let line = "...*..";
        let bot = Some("....10");
        let gear_position = line.chars().position(|x| x == '*').unwrap();

        let result = Solution::find_gear_pair(gear_position, line, top, bot)?;

        assert_eq!(result, Some((123, 10)));

        Ok(())
    }

    #[test]
    fn test_process_part_1_line_ok() -> Result<()> {
        let top = "...*......".to_string();
        let line = "..35..633.".to_string();
        let bottom = "......#...".to_string();

        let expected = vec![35, 633];

        assert_eq!(
            Solution::process_part_1_line(&line, Some(&top), Some(&bottom))?,
            expected
        );

        Ok(())
    }

    #[test]
    fn test_part_1_file_ok() -> Result<()> {
        let engine_map = load_contents()?;

        let solution = Solution::part_1(&engine_map)?;

        assert_eq!(solution, 553079);

        Ok(())
    }

    #[test]
    fn test_part_1_example_ok() -> Result<()> {
        assert_eq!(Solution::part_1(&example_engine())?, 4361);

        Ok(())
    }

    fn load_contents() -> Result<Vec<String>> {
        let file = File::open("./data/input")?;

        let lines = BufReader::new(file).lines();
        let mut buffer = vec![];

        for line in lines {
            buffer.push(line?);
        }

        Ok(buffer)
    }
}
