impl Solution {
    pub fn two_egg_drop(n: i32) -> i32 {
        let k = ((-1.0 + ((1 + 8 * n) as f64).sqrt()) / 2.0).ceil() as i32;
        k
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![(2, 2), (100, 14)];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::two_egg_drop(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
