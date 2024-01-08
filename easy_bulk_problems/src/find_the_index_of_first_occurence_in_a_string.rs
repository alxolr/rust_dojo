struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if haystack.len() < needle.len() {
            return -1;
        }

        let mut result_idx = -1;

        let haystack = haystack.chars().collect::<Vec<_>>();
        let needle = needle.chars().collect::<Vec<_>>();

        let mut i = 0;
        'main: while i < haystack.len() {
            // we may have a potential starting sequence
            if haystack[i] == needle[0] {
                result_idx = i as i32;

                let mut k = i + 1;
                let mut j = 1;

                while j < needle.len() {
                    if k >= haystack.len() || needle[j] != haystack[k] {
                        // the sequence is broken
                        i += 1;
                        result_idx = -1;
                        continue 'main;
                    }
                    j += 1;
                    k += 1;
                }

                break;
            }

            i += 1;
        }

        result_idx
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_content_children_ok() {
        let scenarios = vec![
            (("sadbutsad".to_string(), "sad".to_string()), 0),
            (("leetcode".to_string(), "leeto".to_string()), -1),
            (("aaa".to_string(), "aaaa".to_string()), -1),
            (("mississippi".to_string(), "issipi".to_string()), -1),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((haystack, needle), expected))| {
                let result = Solution::str_str(haystack, needle);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
