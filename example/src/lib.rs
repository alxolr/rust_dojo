pub struct Solution;

pub fn dfs(idx: usize, sols: &mut Vec<Vec<i32>>, nums: &[i32]) {
    let mut level_sols = vec![];
    
    for sol in sols.iter() {
        let mut temp = sol.clone();
        temp.push(nums[idx]);

        level_sols.push(temp);
    }

    if !level_sols.is_empty() {
        sols.append(&mut level_sols);
    }

    sols.push(vec![nums[idx]]);

    if idx + 1 <= nums.len() - 1 {
        dfs(idx + 1, sols, nums);
    }
}

impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        let mut combos = vec![];
        dfs(0, &mut combos, &nums);

        combos
            .iter()
            .map(|x| {
                x.iter().fold(0, |mut acc, it| {
                    acc ^= *it;

                    acc
                })
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_ok() {
        let scenarios = vec![
            (vec![1, 3], 6),
            (vec![5, 1, 6], 28),
            (vec![3, 4, 5, 6, 7, 8], 480),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::subset_xor_sum(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
