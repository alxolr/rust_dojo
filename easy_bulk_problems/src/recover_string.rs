pub struct Solution;

impl Solution {
    pub fn restore_string(s: String, indices: Vec<i32>) -> String {
        let mut zipped = s.chars().zip(indices.iter()).collect::<Vec<_>>();
        zipped.sort_by(|a, b| a.1.cmp(b.1));

        zipped.iter().map(|(ch, _)| ch).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_ok() {
        let s = "codeleet".to_string();
        let indices = vec![4, 5, 6, 7, 0, 2, 1, 3];

        assert_eq!(Solution::restore_string(s, indices), "leetcode".to_string());
    }
}
