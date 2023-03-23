/// Implement the Trie data structure

// impl Trie {

//     fn new() -> Self {
//     }
//     fn insert(&self, word: String) {
//     }
//     fn search(&self, word: String) -> bool {
//     }
//     fn starts_with(&self, prefix: String) -> bool {
//     }
// }

struct Trie {
    root: TrieNode,
}

#[derive(Default, Clone)]
struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    is_word: bool,
}

impl Trie {
    fn new() -> Self {
        Trie {
            root: TrieNode {
                children: [None; 26],
                is_word: false,
            },
        }
    }

    fn insert(&mut self, word: String) {
        let mut node = &mut self.root;
        for c in word.chars() {
            let index = (c as u8 - b'a') as usize;

            if node.children[index].is_none() {
                node.children[index] = Some(Box::new(TrieNode {
                    children: [None.clone(); 26],
                    is_word: false,
                }));
            }

            node = node.children[index].as_mut().unwrap();
        }
        node.is_word = true;
    }

    fn search(&self, word: String) -> bool {
        let mut node = &self.root;
        for c in word.chars() {
            let index = (c as u8 - b'a') as usize;
            if node.children[index].is_none() {
                return false;
            }
            node = node.children[index].as_ref().unwrap();
        }
        node.is_word
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut node = &self.root;
        for c in prefix.chars() {
            let index = (c as u8 - b'a') as usize;
            if node.children[index].is_none() {
                return false;
            }
            node = node.children[index].as_ref().unwrap();
        }
        true
    }
}

// /**
//  * Your Trie object will be instantiated and called as such:
//  * let obj = Trie::new();
//  * obj.insert(word);
//  * let ret_2: bool = obj.search(word);
//  * let ret_3: bool = obj.starts_with(prefix);
//  */`
