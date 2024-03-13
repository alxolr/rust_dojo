struct Solution;
use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut task_counts = vec![0; 26];
        let mut priority_queue = BinaryHeap::new();

        for task in tasks {
            let idx = (task as u8 - b'A') as usize;
            let priority = task_counts[idx] * (n + 1);

            task_counts[idx] += 1;
            priority_queue.push(Reverse(priority));
        }

        let mut cycles = 0;

        while !priority_queue.is_empty() {
            if let Some(peek) = priority_queue.peek() {
                if peek.0 <= cycles {
                    priority_queue.pop();
                }
            }

            cycles += 1;
        }

        cycles
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            ((vec!['A', 'A', 'A', 'B', 'B', 'B'], 2), 8),
            ((vec!['A', 'A', 'A', 'B', 'B', 'B'], 0), 6),
            (
                (
                    vec!['A', 'A', 'A', 'A', 'A', 'A', 'B', 'C', 'D', 'E', 'F', 'G'],
                    2,
                ),
                16,
            ),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((tasks, n), expected))| {
                let result = Solution::least_interval(tasks, n);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
