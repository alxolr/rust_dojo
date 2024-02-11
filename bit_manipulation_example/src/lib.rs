struct Solution;

impl Solution {
    pub fn find_k_or(nums: Vec<i32>, k: i32) -> i32 {
        let mut bit_counter = vec![0; 32];

        for num in nums {
            count_bits(num, &mut bit_counter);
        }

        bit_counter
            .iter()
            .enumerate()
            .filter(|(_, count)| *count >= &k)
            .fold(0, |mut acc, (idx, _)| {
                acc += 2i32.pow(idx as u32);

                acc
            })
    }
}

fn count_bits(mut num: i32, bit_counter: &mut Vec<i32>) {
    let mut bit_idx = 0;

    while num > 0 {
        let bit_value = num % 2;

        if bit_value == 1 {
            bit_counter[bit_idx] += 1;
        }
        bit_idx += 1;

        num /= 2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_ok() {
        let scenarios = vec![
            ((vec![7, 12, 9, 8, 9, 15], 4), 9),
            ((vec![2, 12, 1, 11, 4, 5], 6), 0),
            ((vec![10, 8, 5, 9, 11, 6, 8], 1), 15),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((nums, k), expected))| {
                let result = Solution::find_k_or(nums, k);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
