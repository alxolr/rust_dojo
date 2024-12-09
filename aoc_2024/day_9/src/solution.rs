use std::collections::VecDeque;

use crate::error::Result;
pub struct Solution;

impl Solution {
    pub fn part_1(input: &str) -> Result<i64> {
        let mut current_position = 0;
        let mut current_file_id = 0;

        let mut available_spaces = VecDeque::new();
        let mut file_positions = VecDeque::new();

        for (index, digit) in input.chars().flat_map(|x| x.to_digit(10)).enumerate() {
            let is_file = index % 2 == 0;

            if is_file {
                for _ in 0..digit {
                    file_positions.push_back((current_file_id, current_position));
                    current_position += 1;
                }
                current_file_id += 1;
            } else {
                for _ in 0..digit {
                    available_spaces.push_front(current_position);
                    current_position += 1;
                }
            }
        }

        let mut total_sum = 0;

        while let Some((file_id, file_pos)) = file_positions.pop_back() {
            if let Some(space_pos) = available_spaces.pop_back() {
                if space_pos < file_pos {
                    total_sum += file_id * space_pos;
                } else {
                    total_sum += file_id * file_pos;
                }
            } else {
                total_sum += file_id * file_pos;
            }
        }

        Ok(total_sum)
    }

    pub fn part_2(_input: &str) -> Result<i32> {
        Ok(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() -> Result<()> {
        let input = r#"2333133121414131402"#;
        assert_eq!(Solution::part_1(input)?, 1928);

        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let input = r#""#;
        assert_eq!(Solution::part_2(input)?, 0);

        Ok(())
    }
}

//00...111...2...333.44.5555.6666.777.888899
