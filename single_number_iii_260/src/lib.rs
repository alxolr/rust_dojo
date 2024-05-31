pub struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let xor = nums.iter().fold(0, |mut acc, it| {
            acc ^= it;

            acc
        });

        let mut diff_bit = 1;

        while xor & diff_bit == 0 {
            diff_bit <<= 1;
        }

        let mut first = 0;
        let mut second = 0;

        for num in nums {
            if num & diff_bit != 0 {
                first ^= num;
            } else {
                second ^= num;
            }
        }

        vec![first, second]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            (vec![1, 2, 1, 3, 2, 5], vec![3, 5]),
            (vec![-1, 0], vec![-1, 0]),
            (vec![0, 1], vec![1, 0]),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::single_number(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
