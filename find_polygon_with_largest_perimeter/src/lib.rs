pub struct Solution;

impl Solution {
    pub fn largest_perimeter(nums: Vec<i32>) -> i64 {
        let mut nums = nums;
        nums.sort();

        let mut result = -1;

        let mut total = 0;

        for it in nums {
            if total > it {
                result = total + it;
            }

            total += it;
        }


        result as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            (vec![5, 5, 5], 15),
            (vec![1, 12, 1, 2, 5, 50, 3], 12),
            (vec![5, 5, 50], -1),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::largest_perimeter(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
