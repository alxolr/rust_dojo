pub struct Solution;

impl Solution {
    pub fn largest_combination(candidates: Vec<i32>) -> i32 {
        let mut bit_count = vec![0; 24];

        for mut candidate in candidates {
            let mut idx = 0;
            while candidate > 0 {
                bit_count[idx] += (candidate & 1) as usize;
                candidate >>= 1;
                idx += 1;
            }
        }

        *bit_count.iter().max().unwrap_or(&0) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![(vec![16, 17, 71, 62, 12, 24, 14], 4)];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::largest_combination(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
