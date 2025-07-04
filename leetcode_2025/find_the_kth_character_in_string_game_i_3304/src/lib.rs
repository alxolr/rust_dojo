impl Solution {
    pub fn kth_character(k: i32) -> char {
        let mut s = "a".to_string();

        while s.len() < k as usize {
            let next: String = s
                .chars()
                .map(|c| if c == 'z' { 'a' } else { (c as u8 + 1) as char })
                .collect();
            
            s = format!("{}{}", s, next);
        }

        s.chars().nth((k - 1) as usize).unwrap_or('a')
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![(5, 'b'), (10, 'c')];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::kth_character(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
