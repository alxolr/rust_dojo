use std::collections::HashSet;

struct Solution;

impl Solution {
    fn two_sum(nums: &Vec<&i32>, target: i32) -> Vec<Vec<i32>> {
        let mut left = 0;
        let mut right = nums.len() - 1;
        let mut solutions = vec![];

        while left < right {
            if nums[left] + nums[right] > target {
                if right == 0 {
                    break;
                }

                right -= 1;
            } else if nums[left] + nums[right] < target {
                left += 1;
            } else {
                solutions.push(vec![*nums[left], *nums[right]]);
                left += 1;

                if right == 0 {
                    break;
                }

                right -= 1;
            }
        }

        solutions
    }

    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        let mut results = vec![];
        let mut set = HashSet::new();

        nums.sort_unstable();

        for (idx, num) in nums.iter().enumerate() {
            let target = -1 * num;

            let range_nums = nums[0..idx]
                .iter()
                .chain(nums[idx + 1..nums.len()].iter())
                .collect::<Vec<_>>();

            let solutions: Vec<Vec<i32>> = Solution::two_sum(&range_nums, target);

            if !solutions.is_empty() {
                for mut solution in solutions {
                    solution.push(*num);

                    solution.sort_unstable();

                    if !set.contains(&solution) {
                        results.push(solution.clone());
                        set.insert(solution);
                    }
                }
            }
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            // (vec![-1, 0, 1, 2, -1, -4], vec![vec![-1, 0, 1]]),
            (
                vec![-1, 0, 1, 2, -1, -4, -2, -3, 3, 0, 4],
                vec![vec![-1, 0, 1]],
            ),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::three_sum(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
