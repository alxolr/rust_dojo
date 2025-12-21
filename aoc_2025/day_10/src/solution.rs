use std::{ 
    cmp::Ordering,
    collections::{BinaryHeap, HashSet, VecDeque},
    fmt::Debug,
    hash::Hash,
};

use rayon::{iter::ParallelIterator, str::ParallelString};

use crate::error::Result;
pub struct Solution;

#[derive(Eq, Ord, Debug)]
struct HeuristicState<T>(u16, T);

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
        let xor_state_change_fn = |a: &u16, b: &u16| -> u16 { a ^ b };
        let empty_heuristic_fn = |_: &u16, _: &u16| -> u16 { 1 }; // we don't care about  heuristic in here
        let is_valid_transition_fn = |_: &u16, _: &u16| -> bool { true };

        let answer = parse_part_1(input)
            .map(|(initial_state, masks)| {
                a_star_traversal(
                    initial_state,
                    0u16,
                    &masks,
                    is_valid_transition_fn,
                    xor_state_change_fn,
                    empty_heuristic_fn,
                )
            })
            .sum();

        Ok(answer)
    }

    pub fn part_2(input: &str) -> Result<u64> {
        let answer = parse_part_2(input)
            .map(|(buttons, target_state)| solve_linear_system(buttons, target_state))
            .sum();

        Ok(answer)
    }
}

// Count the minimum amount of changes
fn a_star_traversal<T, F, H, E>(
    start_state: T,
    target_state: T,
    transitions: &Vec<T>,
    is_valid_fn: E, // we will be filtering states that exceed the value
    state_change_fn: F,
    heuristic_fn: H, // it's used to sort the output given a value which makes the current state closer to the target
) -> u64
where
    T: Hash + Eq + PartialEq + Clone + Ord + Debug,
    F: Fn(&T, &T) -> T,
    H: Fn(&T, &T) -> u16,
    E: Fn(&T, &T) -> bool,
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
                if !visited_states.contains(&transition) && is_valid_fn(&transition, &target_state)
                {
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

fn solve_linear_system(buttons: Vec<Vec<u16>>, target: Vec<u16>) -> u64 {
    let matrix = Matrix::from_problem(&buttons, &target);
    
    // DFS over the reduced solution space
    let max = target.iter().max().copied().unwrap_or(0) as usize + 1;
    let mut min = usize::MAX;
    let mut values = vec![0; matrix.independents.len()];
    
    dfs(&matrix, 0, &mut values, &mut min, max);
    
    min as u64
}

const EPSILON: f64 = 1e-9;

struct Matrix {
    data: Vec<Vec<f64>>,
    rows: usize,
    cols: usize,
    dependents: Vec<usize>,
    independents: Vec<usize>,
}

impl Matrix {
    fn from_problem(buttons: &[Vec<u16>], target: &[u16]) -> Self {
        let rows = target.len();
        let cols = buttons.len();
        let mut data = vec![vec![0.0; cols + 1]; rows];

        // Build coefficient matrix: A[i][j] = 1 if button j affects counter i
        for (c, button) in buttons.iter().enumerate() {
            for &r in button {
                let r = r as usize;
                if r < rows {
                    data[r][c] = 1.0;
                }
            }
        }

        // Add target values to the last column
        for (r, &val) in target.iter().enumerate() {
            data[r][cols] = val as f64;
        }

        let mut matrix = Self {
            data,
            rows,
            cols,
            dependents: Vec::new(),
            independents: Vec::new(),
        };

        matrix.gaussian_elimination();
        matrix
    }

    // https://en.wikipedia.org/wiki/Gaussian_elimination
    fn gaussian_elimination(&mut self) {
        let mut pivot = 0;
        let mut col = 0;

        while pivot < self.rows && col < self.cols {
            // Find the best pivot row for this column
            let (best_row, best_value) = self
                .data
                .iter()
                .enumerate()
                .skip(pivot)
                .map(|(r, row)| (r, row[col].abs()))
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                .unwrap();

            // If the best value is zero, this is a free variable
            if best_value < EPSILON {
                self.independents.push(col);
                col += 1;
                continue;
            }

            // Swap rows and mark this column as dependent
            self.data.swap(pivot, best_row);
            self.dependents.push(col);

            // Normalize pivot row
            let pivot_value = self.data[pivot][col];
            for val in &mut self.data[pivot][col..=self.cols] {
                *val /= pivot_value;
            }

            // Eliminate this column in all other rows
            for r in 0..self.rows {
                if r != pivot {
                    let factor = self.data[r][col];
                    if factor.abs() > EPSILON {
                        let pivot_row = self.data[pivot][col..=self.cols].to_vec();
                        self.data[r][col..=self.cols]
                            .iter_mut()
                            .zip(&pivot_row)
                            .for_each(|(val, &pivot_val)| {
                                *val -= factor * pivot_val;
                            });
                    }
                }
            }

            pivot += 1;
            col += 1;
        }

        // Any remaining columns are free variables
        self.independents.extend(col..self.cols);
    }

    // Check if the given values for independent variables are valid
    fn valid(&self, values: &[usize]) -> Option<usize> {
        // Start with how many times we've pressed the free variables
        let mut total = values.iter().sum::<usize>();

        // Calculate dependent variable values based on independent variables
        for row in 0..self.dependents.len() {
            // Calculate this dependent by subtracting the sum of free variable pushes from the solution
            let val = self
                .independents
                .iter()
                .enumerate()
                .fold(self.data[row][self.cols], |acc, (i, &col)| {
                    acc - self.data[row][col] * (values[i] as f64)
                });

            // We need non-negative, whole numbers for a valid solution
            if val < -EPSILON {
                return None;
            }
            let rounded = val.round();
            if (val - rounded).abs() > EPSILON {
                return None;
            }

            total += rounded as usize;
        }

        Some(total)
    }
}

fn dfs(matrix: &Matrix, idx: usize, values: &mut [usize], min: &mut usize, max: usize) {
    // When we've assigned all independent variables, check if it's a valid solution
    if idx == matrix.independents.len() {
        if let Some(total) = matrix.valid(values) {
            *min = (*min).min(total);
        }
        return;
    }

    // Try different values for the current independent variable
    let total: usize = values[..idx].iter().sum();
    for val in 0..max {
        // Optimization: If we ever go above our min, we can't possibly do better
        if total + val >= *min {
            break;
        }
        values[idx] = val;
        dfs(matrix, idx + 1, values, min, max);
    }
}

fn parse_part_2(input: &str) -> impl ParallelIterator<Item = (Vec<Vec<u16>>, Vec<u16>)> + use<'_> {
    input.par_lines().map(|line| {
        let line_parts = line.trim().split_whitespace().collect::<Vec<_>>();
        let count = line_parts.len();

        let transitions = line_parts.iter().skip(1).take(count - 2);
        let transitions = transitions
            .map(|transition| {
                transition
                    .replace("(", "")
                    .replace(")", "")
                    .split(",")
                    .flat_map(|num| num.parse::<u16>())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<Vec<u16>>>();

        let state = line_parts[count - 1]
            .replace("{", "")
            .replace("}", "")
            .split(",")
            .flat_map(|num| num.parse::<u16>())
            .collect::<Vec<_>>();

        (transitions, state)
    })
}

fn parse_part_1(input: &str) -> impl Iterator<Item = (u16, Vec<u16>)> + use<'_> {
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
        assert_eq!(Solution::part_2(input)?, 10);

        let input = r#"[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}"#;
        assert_eq!(Solution::part_2(input)?, 12);

        let input = r#"[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"#;
        assert_eq!(Solution::part_2(input)?, 11);
        
        // Test all three examples together
        let input = r#"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"#;
        assert_eq!(Solution::part_2(input)?, 33);

        Ok(())
    }

    #[test]
    fn test_first_input_line() -> Result<()> {
        let input = r#"[.##......] (0,1,3,4,6,7,8) (1,2,3,5,6,8) (0,1) (3,5,6,7) (2,5,7) (1,2,3,4,5,7,8) (7) (0,1,3) (0,3,7) (1,4,6) {36,63,29,56,28,48,43,52,23}"#;
        let result = Solution::part_2(input)?;
        println!("First line result: {}", result);
        Ok(())
    }
}
