use std::collections::{HashMap, HashSet};

pub fn word_pattern(pattern: String, words: String) -> bool {
    let mut dictionary: HashMap<char, &str> = HashMap::new();
    let mut seen: HashSet<&str> = HashSet::new();

    let pattern_len = pattern.len();
    let words_iter = words.split_whitespace();
    let words_count = words_iter.clone().count();

    let pattern_iter = pattern.chars();

    if words_count != pattern_len {
        return false;
    }

    let iter = words_iter.zip(pattern_iter);

    for (word, ch) in iter {
        if !seen.contains(word) {
            seen.insert(word);
            let entry = dictionary.entry(ch).or_insert(word);
            if *entry != word {
                return false;
            }
        } else {
            if let Some(entry) = dictionary.get(&ch) {
                if *entry != word {
                    return false;
                }
            } else {
                return false;
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_one() {
        let result = word_pattern("abba".to_string(), "dog cat cat dog".to_string());
        assert_eq!(result, true);
    }

    #[test]
    fn test_case_two() {
        let result = word_pattern("abba".to_string(), "dog cat cat fish".to_string());
        assert_eq!(result, false);
    }

    #[test]
    fn test_case_three() {
        let result = word_pattern("aaaa".to_string(), "dog cat cat dog".to_string());
        assert_eq!(result, false);
    }

    #[test]
    fn test_case_four() {
        let result = word_pattern("abba".to_string(), "dog dog dog dog".to_string());
        assert_eq!(result, false);
    }

    #[test]
    fn test_case_five() {
        let result = word_pattern("aaa".to_string(), "aa aa aa aa".to_string());
        assert_eq!(result, false);
    }
}
