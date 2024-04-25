pub struct Solution;

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut res = vec![];
        let mut part = vec![];

        Self::dfs(0, &s, &mut part, &mut res);

        res
    }

    fn dfs(idx: usize, s: &str, part: &mut Vec<String>, res: &mut Vec<Vec<String>>) {
        if idx >= s.len() {
            res.push(part.clone());
            return;
        }

        for j in idx..s.len() {
            if Self::is_palindrome(&s[idx..=j]) {
                part.push(s[idx..=j].to_string());
                Self::dfs(j + 1, s, part, res);
                part.pop();
            }
        }
    }

    fn is_palindrome(s: &str) -> bool {
        let (mut i, mut j) = (0, s.len() - 1);

        while i < j {
            if s.as_bytes()[i] != s.as_bytes()[j] {
                return false;
            }
            i += 1;
            j -= 1;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = "aab".to_string();
        let res = Solution::partition(s);
        assert_eq!(res, vec![vec!["a", "a", "b"], vec!["aa", "b"]]);
    }
}
