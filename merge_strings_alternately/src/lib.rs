pub struct Solution;
impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut result = String::new();
        let mut i = 0;
        let mut j = 0;
        while i < word1.len() && j < word2.len() {
            result.push(word1.chars().nth(i).unwrap());
            result.push(word2.chars().nth(j).unwrap());
            i += 1;
            j += 1;
        }
        if i < word1.len() {
            result.push_str(&word1[i..]);
        }
        if j < word2.len() {
            result.push_str(&word2[j..]);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::merge_alternately("abc".to_string(), "pqr".to_string()),
            "apbqcr".to_string()
        );
        assert_eq!(
            Solution::merge_alternately("ab".to_string(), "pqrs".to_string()),
            "apbqrs".to_string()
        );
        assert_eq!(
            Solution::merge_alternately("abcd".to_string(), "pq".to_string()),
            "apbqcd".to_string()
        );
    }
}
