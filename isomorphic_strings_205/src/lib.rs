use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let iter = s.as_bytes().iter().zip(t.as_bytes().iter());
        let mut dictionary = HashMap::new();
        let mut reversed_dictionary = HashMap::new();

        for (l, r) in iter {
            if let Some(val) = dictionary.get(l) {
                if val != r {
                    return false;
                }
            } else if reversed_dictionary.get(r).is_none() {
                dictionary.insert(*l, *r);
                reversed_dictionary.insert(*r, *l);
            } else {
                let val = reversed_dictionary.get(r).unwrap();
                if val != l {
                    return false;
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            (("badc".to_string(), "baba".to_string()), false),
            (("egg".to_string(), "add".to_string()), true),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((s, t), expected))| {
                let result = Solution::is_isomorphic(s, t);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
