use std::collections::HashMap;

struct Solution;

#[derive(Debug)]
pub struct TrieNode {
    children: HashMap<char, (TrieNode, usize)>,
    is_end: bool,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            is_end: false,
        }
    }
}

#[derive(Debug)]
pub struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }

    fn insert(&mut self, word: &str) {
        let mut current = &mut self.root;

        for ch in word.chars() {
            let mut entry = current.children.entry(ch).or_insert((TrieNode::new(), 0));
            entry.1 += 1;

            current = &mut entry.0;
        }

        current.is_end = true;
    }
}

impl Solution {
    pub fn longest_common_prefix(words: Vec<String>) -> String {
        let mut trie = Trie::new();
        let len = words.len();

        for word in words {
            trie.insert(&word);
        }

        let mut prefix = "".to_string();
        let mut curr = &trie.root;

        loop {
            if &curr.children.len() != &1 {
                break;
            }

            for (ch, (next_node, size)) in curr.children.iter() {
                if len == *size {
                    prefix.push(*ch);
                }
                curr = next_node;
            }
        }

        prefix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_ok() {
        let scenarios = vec![
            (
                vec![
                    "flower".to_string(),
                    "flow".to_string(),
                    "flight".to_string(),
                ],
                "fl".to_string(),
            ),
            (
                vec!["abab".to_string(), "aba".to_string(), "".to_string()],
                "".to_string(),
            ),
            (
                vec!["dog".to_string(), "racecar".to_string(), "car".to_string()],
                "".to_string(),
            ),
            (vec!["".to_string(), "b".to_string()], "".to_string()),
            (vec!["b".to_string(), "bcb".to_string()], "b".to_string()),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (s, expected))| {
                let result = Solution::longest_common_prefix(s);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
