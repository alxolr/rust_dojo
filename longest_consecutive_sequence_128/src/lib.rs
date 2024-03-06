use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let set = nums.iter().collect::<HashSet<&i32>>();
        let mut seen = HashSet::new();
        let mut max_sequence_len = 0;

        for num in &nums {
            if seen.contains(num) {
                continue;
            }

            if !set.contains(&(num - 1)) {
                // start of the sequence
                let mut seq_length = 1;

                let mut tmp = *num;

                while set.contains(&(tmp + 1)) == true {
                    tmp += 1;
                    seen.insert(tmp);
                    seq_length += 1;
                }

                max_sequence_len = max_sequence_len.max(seq_length)
            }
        }

        max_sequence_len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            (vec![100, 4, 200, 1, 3, 2], 4),
            (vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1], 9),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::longest_consecutive(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
