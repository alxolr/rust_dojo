pub struct Solution;

const AB: &str = "AB";
const CD: &str = "CD";

impl Solution {
    fn contains(buf: &[char], pat: &str) -> bool {
        buf.len() >= 2 && buf[buf.len() - 2..] == pat.chars().collect::<Vec<_>>()[..]
    }

    pub fn min_length(s: String) -> i32 {
        let mut buf: Vec<char> = vec![];

        for ch in s.chars() {
            buf.push(ch);

            while buf.len() >= 2 {
                if Solution::contains(&buf, AB) || Solution::contains(&buf, CD) {
                    buf.pop();
                    buf.pop();
                } else {
                    break;
                }
            }
        }

        buf.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![("ABFCACDB".to_string(), 2), ("ACBBD".to_string(), 5)];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::min_length(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
