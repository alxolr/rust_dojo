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
        // get the root node as a pointer called current
        // iterate through the trie data structure, getting next child
        let mut current = &mut self.root;

        for ch in word.chars() {
            // for each character in the word we check if we already have a trie node
            // started at this char, if yes then we continue if no then we start a new node
            let child = current
                .children
                .entry(ch)
                .or_insert(Box::new(TrieNode::new()));

            // we navigate through the trie tree structure using the current/child pair
            current = child;
        }

        // mark the node at the last letter to be true
        current.is_end = true;
    }

    fn shortest_root(&self, word: &str) -> String {
        // create a buffer to store the progress of already found characters
        let mut buffer = String::new();

        // use the same current/child way to navigate through the (trie) tree
        let mut current = &self.root;

        // check all the chars in current word
        for ch in word.chars() {
            // if we found a letter in the trie we append it to the existing buffer
            if let Some(child) = current.children.get(&ch) {
                buffer.push(ch);

                // once the node is an end word we need to return accumulated buffer as this being the
                // shortest root for the word
                if child.is_end {
                    return buffer;
                }

                current = child;
            } else {
                // if the letter was not found this means there is no prefix root for this word
                // return an empty string
                return String::new();
            }
        }

        return buffer;
    }
}

impl Solution {
    pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
        let mut trie = Trie::new();
        let words: Vec<_> = sentence.split_whitespace().collect();
        let mut result: Vec<_> = Vec::with_capacity(words.len());

        // Add all words in the dictioary to the Trie Data structure
        for word in dictionary {
            trie.insert(&word);
        }

        for word in words {
            // get the shortest root, output will be either empty string or a word from the dictionary
            let shortest_root = trie.shortest_root(word);

            if shortest_root.is_empty() {
                result.push(word.to_string());
            } else {
                result.push(shortest_root.to_string());
            }
        }

        // join all the words to return the new sentence
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
