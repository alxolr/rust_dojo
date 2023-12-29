struct Solution;

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut result = vec![0; n as usize + 1];
        let mut range = 0; // this will be the exponent of two

        


        for i in 0..=n as usize {
            result[i] = (i as i32).count_ones() as i32;
        }

        result
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_bits_ok() {
        let scenarios = vec![(2, vec![0, 1, 1]), (5, vec![0, 1, 1, 2, 1, 2])];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::count_bits(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
