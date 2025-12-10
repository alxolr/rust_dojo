use std::collections::{HashSet, VecDeque};

use crate::error::Result;
pub struct Solution;

impl Solution {
    pub fn part_1(input: &str) -> Result<u64> {
        let answer = parse_input(input)
            .map(|(state, masks, _)| solve_part_1(state, &masks))
            .sum();

        Ok(answer)
    }

    pub fn part_2(input: &str) -> Result<u64> {
        let answer = 1;
        Ok(answer)
    }
}

fn solve_part_1(initial_state: u16, masks: &Vec<u16>) -> u64 {
    // we treat the state as a u16 number, and the masks as well, we will use xor to inverse the bits
    // we will use a breadth first search algorithm to find the number of iterations
    // we go backwards from our state to state ....... -> which is zero

    let mut visited_states = HashSet::new();
    let mut state_machine = VecDeque::<(usize, u16)>::new();
    state_machine.push_front((0, initial_state));
    visited_states.insert(initial_state);
    let target_state = 0u16;

    while let Some((clicks, state)) = state_machine.pop_front() {
        if state == target_state {
            return clicks as u64;
        }

        for mask in masks.iter() {
            let new_state = state ^ mask;
            if !visited_states.contains(&new_state) {
                visited_states.insert(new_state);
                state_machine.push_back((clicks + 1, new_state));
            }
        }
    }

    panic!("Expected to finish by now")
}

fn parse_input(input: &str) -> impl Iterator<Item = (u16, Vec<u16>, Vec<u16>)> + use<'_> {
    input.lines().map(|line| {
        // let mut parts = line.trim().split_whitespace();
        let line_parts = line.trim().split_whitespace().collect::<Vec<_>>();
        let count = line_parts.len();

        let state = line_parts[0]
            .replace("[", "")
            .replace("]", "")
            .replace(".", "0")
            .replace("#", "1")
            .chars()
            .rev()
            .collect::<String>();

        let masks = line_parts.iter().skip(1).take(count - 2);

        let joltages = line_parts[count - 1]
            .replace("{", "")
            .replace("}", ",")
            .split(",")
            .flat_map(|joltage| joltage.parse::<u16>())
            .collect();

        let state = u16::from_str_radix(&state, 2).unwrap_or_default();
        let masks = masks
            .flat_map(|mask| {
                let mask = mask
                    .replace("(", "")
                    .replace(")", "")
                    .split(",")
                    .flat_map(|num| num.parse::<usize>())
                    .fold(vec!['0'; 16], |mut acc, num| {
                        acc[num] = '1';
                        acc
                    })
                    .iter()
                    .rev()
                    .collect::<String>();

                u16::from_str_radix(&mask, 2)
            })
            .collect::<Vec<u16>>();

        (state, masks, joltages)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() -> Result<()> {
        let input = r#"[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"#;
        assert_eq!(Solution::part_1(input)?, 2);

        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let input = r#"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}"#;
        assert_eq!(Solution::part_2(input)?, 0);

        Ok(())
    }
}
