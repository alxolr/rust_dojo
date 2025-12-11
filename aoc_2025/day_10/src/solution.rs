use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet, VecDeque},
    hash::Hash,
};

use crate::error::Result;
pub struct Solution;

#[derive(Eq, Ord)]
struct HeuristicState<T>(u64, T);

impl<T> PartialEq for HeuristicState<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> PartialOrd for HeuristicState<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Solution {
    pub fn part_1(input: &str) -> Result<u64> {
        let xor_state_transition_fn = |a: &u16, b: &u16| -> u16 { a ^ b };
        let empty_heuristic_fn = |_: &u16, _: &u16| -> u64 { 1 }; // we don't care about  heuristic in here

        let answer = parse_input(input)
            .map(|(initial_state, masks)| {
                a_star_traversal(
                    initial_state,
                    0u16,
                    &masks,
                    xor_state_transition_fn,
                    empty_heuristic_fn,
                )
            })
            .sum();

        Ok(answer)
    }

    pub fn part_2(_input: &str) -> Result<u64> {
        let answer = 1;
        Ok(answer)
    }
}

// Count the minimum amount of changes
fn a_star_traversal<T, F, H>(
    start_state: T,
    target_state: T,
    transitions: &Vec<T>,
    state_change_fn: F,
    heuristic_fn: H, // it's used to sort the output given a value which makes the current state closer to the target
) -> u64
where
    T: Hash + Eq + PartialEq + Clone + Ord,
    F: Fn(&T, &T) -> T,
    H: Fn(&T, &T) -> u64,
{
    // we treat the state as a u16 number, and the masks as well, we will use xor to inverse the bits
    // we will use a breadth first search algorithm to find the number of iterations
    // we go backwards from our state to state ....... -> which is zero
    let mut visited_states = HashSet::new();
    let mut state_machine = VecDeque::<(usize, T)>::new();
    state_machine.push_front((0, start_state.clone()));
    visited_states.insert(start_state);

    while let Some((clicks, state)) = state_machine.pop_front() {
        if state == target_state {
            return clicks as u64;
        }

        let mut heuristic_states = transitions
            .iter()
            .map(|transition| state_change_fn(&state, transition))
            .filter_map(|transition| {
                if !visited_states.contains(&transition) {
                    let heuristic = heuristic_fn(&transition, &target_state);
                    Some(HeuristicState(heuristic, transition))
                } else {
                    None
                }
            })
            .collect::<BinaryHeap<HeuristicState<T>>>();

        while let Some(HeuristicState(_, new_state)) = heuristic_states.pop() {
            visited_states.insert(new_state.clone());
            state_machine.push_back((clicks + 1, new_state));
        }
    }

    panic!("Expected to finish by now")
}

fn parse_input(input: &str) -> impl Iterator<Item = (u16, Vec<u16>)> + use<'_> {
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

        (state, masks)
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
