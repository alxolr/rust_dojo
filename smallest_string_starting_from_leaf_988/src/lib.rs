pub struct Solution;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, path: Vec<char>, result: &mut Vec<String>) {
        match node {
            Some(node) => {
                let node_ref = node.as_ref().borrow();
                let mut current_path = path.clone();
                let character = ('a' as u8 + node_ref.val as u8) as char;
                current_path.push(character);

                // If the node is a leaf node (no children), add the path to the result
                if node_ref.left.is_none() && node_ref.right.is_none() {
                    let string = current_path.iter().rev().collect::<String>();
                    result.push(string);
                } else {
                    Self::dfs(&node_ref.left, current_path.clone(), result);
                    Self::dfs(&node_ref.right, current_path.clone(), result)
                }
            }
            None => {}
        }
    }

    pub fn smallest_from_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut strings = vec![];

        // Perform depth-first search on the tree
        Self::dfs(&root, vec![], &mut strings);

        strings.sort();

        if let Some(smallest_string) = strings.first() {
            smallest_string.clone()
        } else {
            "".to_string()
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::smallest_from_leaf(None), "".to_string());

        let tree = Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            }))),
        })));

        assert_eq!(Solution::smallest_from_leaf(tree), "dba".to_string());
    }
}
