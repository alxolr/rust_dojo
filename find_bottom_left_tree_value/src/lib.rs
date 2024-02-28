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
struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut value = (-1, -1);
        Self::dfs(&root, 0, &mut value);

        value.1
    }

    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, depth: i32, height: &mut (i32, i32)) {
        match node {
            Some(node) => {
                let node = node.borrow();
                if depth > height.0 {
                    height.0 = depth;
                    height.1 = node.val;
                }

                Self::dfs(&node.left, depth + 1, height);
                Self::dfs(&node.right, depth + 1, height);
            }
            _ => {}
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        })));

        let result = Solution::find_bottom_left_value(root);

        assert_eq!(result, 1);
    }
}
