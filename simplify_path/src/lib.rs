struct Solution;

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut stack = Vec::new();
        let mut path = path.split('/').filter(|s| !s.is_empty());
        while let Some(s) = path.next() {
            match s {
                "." => {}
                ".." => {
                    stack.pop();
                }
                _ => stack.push(s),
            }
        }
        let mut result = String::new();
        for s in stack {
            result.push('/');
            result.push_str(s);
        }
        if result.is_empty() {
            result.push('/');
        }
        result
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            ("/home/".to_string(), "/home".to_string()),
            ("/../".to_string(), "/".to_string()),
            ("/home//foo/".to_string(), "/home/foo".to_string()),
            ("/a/./b/../../c/".to_string(), "/c".to_string()),
            ("/a/../../b/../c//.//".to_string(), "/c".to_string()),
            ("/a//b////c/d//././/..".to_string(), "/a/b/c".to_string()),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::simplify_path(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
