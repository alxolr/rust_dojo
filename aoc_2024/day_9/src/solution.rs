use std::collections::VecDeque;

use crate::error::Result;
pub struct Solution;


impl Solution {
    pub fn part_1(input: &str) -> Result<i64> {
        // Main idea I will be using a two queues one for the free spaces and the other for the file blocks
        // And for each file block will try to get a space from the queue of spaces
        let mut current_position = 0;
        let mut current_file_id = 0;

        let mut available_spaces = VecDeque::new();
        let mut file_positions: VecDeque<(i64, i64)> = VecDeque::new();

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

    pub fn part_2(input: &str) -> Result<u64> {
        // Same idea for the part two but we will store a bit more information
        // For the files, we will store a triple of (file_id, start_pos, block_count)
        // For the available spaces a tuple of (start_pos, block_count)
        // The algorithm will be for each file_triple, try to find a better start_pos
        // this means check through every available memory

        let mut current_position: u64 = 0;
        let mut current_file_id: u64 = 0;

        let mut available_spaces = VecDeque::new();
        let mut file_positions = VecDeque::new();

        for (index, block_count) in input.chars().flat_map(|x| x.to_digit(10)).enumerate() {
            let is_file = index % 2 == 0;

            if is_file {
                file_positions.push_back((current_file_id, current_position, block_count));
                current_position += block_count as u64;
                current_file_id += 1;
            } else {
                available_spaces.push_back((current_position, block_count));
                current_position += block_count as u64;
            }
        }

        let mut total_sum: u64 = 0;

        for (file_id, start_pos, block_count) in file_positions.iter_mut().rev() {
            // we know the current file start
            // we try to find a compatible block count in the block of memories

            for (free_start_pos, free_block_count) in available_spaces.iter_mut() {
                if block_count <= free_block_count {
                    // we may have a potential match
                    if *free_start_pos < *start_pos {
                        // do the swap
                        *start_pos = *free_start_pos;

                        *free_block_count -= *block_count;
                        *free_start_pos += *block_count as u64;

                        break;
                    }
                    // compute the sum of the new blocks
                }
            }

            for id in 0..*block_count {
                total_sum += *file_id * (*start_pos + id as u64);
            }
        }

        Ok(total_sum)
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
        let input = r#"2333133121414131402"#;
        assert_eq!(Solution::part_2(input)?, 2858);

        Ok(())
    }
}
