impl Solution {
    pub fn num_sub(s: String) -> i32 {
        let mut prefix = vec![0; s.len()];
        let chs = s.chars().collect::<Vec<_>>();

        for (id, ch) in chs.iter().enumerate() {
            if ch == &'1' {
                prefix[id] = if id > 0 { prefix[id - 1] + 1 } else { 1 };
            }
        }

        (prefix.iter().sum::<i64>() % 1000000007) as i32
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            ("0110111", 9),
            ("101", 2),
            ("111111", 21),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::num_sub(input.to_string());
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
