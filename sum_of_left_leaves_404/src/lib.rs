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
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs(&root, false)
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, is_left: bool) -> i32 {
        match root {
            Some(node) => {
                let borowed = node.as_ref().borrow();

                let mut result = 0;
                if borowed.left.is_none() && borowed.right.is_none() && is_left == true {
                    result += borowed.val;
                }

                return result + Self::dfs(&borowed.left, true) + Self::dfs(&borowed.right, false);
            }
            None => 0,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 20,
                left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
            }))),
        })));

        assert_eq!(Solution::sum_of_left_leaves(root), 24);
    }

    #[test]
    fn test_complex_case_ok() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: 
        })));
    }
}
