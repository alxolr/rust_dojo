pub struct Solution;

impl Solution {
    pub fn find_the_winner(n: i32, k: i32) -> i32 {
        let mut winner = 0;
        for i in 1..=n as usize {
            winner = (winner + k as usize) % i;
        }
        (winner + 1) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![((5, 2), 3)];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((n, k), expected))| {
                let result = Solution::find_the_winner(n, k);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
