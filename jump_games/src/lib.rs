use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let last_idx = nums.len() - 1;
        let mut seen: HashSet<usize> = HashSet::new();
        let mut stack = vec![(0, nums[0])];

        while !stack.is_empty() {
            let (idx, steps) = stack.pop().unwrap();

            if idx + steps as usize >= last_idx {
                return true;
            } else {
                if steps == 0 && stack.is_empty() {
                    break;
                }

                for step in 1..=steps as usize {
                    let id = step + idx;
                    if !seen.contains(&id) {
                        stack.push((id, nums[id]));
                        seen.insert(id);
                    }
                }
            }
        }

        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            (vec![2, 3, 1, 1, 4], true),
            (vec![0], true),
            (vec![3, 2, 1, 0, 4], false),
            (vec![2, 5, 0, 0], true),
            (vec![5, 9, 3, 2, 1, 0, 2, 3, 3, 1, 0, 0], true),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::can_jump(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
