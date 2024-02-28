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
pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max = 0;
        Self::dfs(&root, &mut max);

        max
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, max: &mut i32) -> i32 {
        match root {
            Some(node) => {
                let left_height = Self::dfs(&node.borrow().left, max);
                let right_height = Self::dfs(&node.borrow().right, max);

                let res = 2 + left_height + right_height;
                *max = res.max(*max);

                1 + i32::max(left_height, right_height)
            }
            None => -1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
