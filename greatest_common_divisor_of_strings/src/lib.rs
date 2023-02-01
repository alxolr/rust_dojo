pub struct Solution;

pub fn is_divisible(s: &str, d: &str) -> bool {
    if d.len() > s.len() {
        false
    } else if s == d {
        true
    } else {
        let pace = d.len();
        let mut working_str = s.clone();

        while !working_str.is_empty() {
            if pace > working_str.len() {
                return false;
            }

            let sub_str = &working_str[0..pace];
            if sub_str != d {
                return false;
            }

            working_str = &working_str[pace..working_str.len()];
        }

        true
    }
}

impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        let smallest = if str1.len() < str2.len() {
            str1.clone()
        } else {
            str2.clone()
        };

        let mut greater_divisor = "";
        for i in 0..smallest.len() {
            let sub_str = &smallest[0..=i];

            if is_divisible(&str1, sub_str) && is_divisible(&str2, sub_str) {
                greater_divisor = sub_str;
            }
        }

        greater_divisor.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_divisible_works() {
        let scenarios = vec![
            (("ABABAB", "AB"), true),
            (("ABABAB", "AC"), false),
            (("ABA", "ABA"), true),
            (("ABA", "A"), false),
            (("AAA", "A"), true),
            (("AB", "AABA"), false),
            (("ABA", "AA"), false),
            (("ABC", "ABC"), true),
            (("ABCABC", "ABC"), true),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((s, d), expected))| {
                let result = is_divisible(s, d);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }

    #[test]
    fn it_works() {
        let scenarios = vec![
            (("ABCABC".to_string(), "ABC".to_string()), "ABC".to_string()),
            (("ABABAB".to_string(), "ABAB".to_string()), "AB".to_string()),
            (("LEET".to_string(), "CODE".to_string()), "".to_string()),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((str1, str2), expected))| {
                let result = Solution::gcd_of_strings(str1, str2);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
