use std::collections::HashMap;

pub struct Solution;

pub struct TrieNode {
    children: HashMap<char, Box<TrieNode>>,
    is_end: bool,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            children: HashMap::new(),
            is_end: false,
        }
    }
}

pub struct Trie {
    root: Box<TrieNode>,
}

impl Trie {
    fn new() -> Self {
        Self {
            root: Box::new(TrieNode::new()),
        }
    }

    fn insert(&mut self, word: &str) {
        let mut current = &mut self.root;

        for ch in word.chars() {
            let child = current
                .children
                .entry(ch)
                .or_insert(Box::new(TrieNode::new()));

            current = child;
        }

        current.is_end = true;
    }

    fn shortest_root(&self, word: &str) -> String {
        let mut result = "".to_string();
        let mut current = &self.root;
        let mut is_prefix = true;

        for ch in word.chars() {
            result.push(ch);

            if is_prefix {
                if let Some(child) = current.children.get(&ch) {
                    if child.is_end {
                        break;
                    }

                    current = child;
                } else {
                    is_prefix = false;
                }
            }
        }

        result
    }
}

impl Solution {
    pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
        let mut trie = Trie::new();
        let mut result: Vec<_> = vec![];

        for word in dictionary {
            trie.insert(&word);
        }

        for word in sentence.split_whitespace() {
            let word = trie.shortest_root(word);

            result.push(word);
        }

        result.join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            (
                (
                    vec!["cat".to_string(), "bat".to_string(), "rat".to_string()],
                    "the cattle was rattled by the battery".to_string(),
                ),
                "the cat was rat by the bat".to_string(),
            ),
            (
                (
                    vec!["a".to_string(), "b".to_string(), "c".to_string()],
                    "aadsfasf absbs bbab cadsfafs".to_string(),
                ),
                "a a b c".to_string(),
            ),
            (
                (
                    vec![
                        "a".to_string(),
                        "aa".to_string(),
                        "aaa".to_string(),
                        "aaaa".to_string(),
                    ],
                    "a aa a aaaa aaa aaa aaa aaaaaa bbb baba ababa".to_string(),
                ),
                "a a a a a a a a bbb baba a".to_string(),
            ),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((dictionary, sentence), expected))| {
                let result = Solution::replace_words(dictionary, sentence);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
