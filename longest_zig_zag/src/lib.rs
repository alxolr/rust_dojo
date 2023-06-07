use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;
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
impl Solution {
    pub fn longest_zig_zag(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max = 0;
        Self::dfs(&root, &mut max, true);

        max
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, max: &mut i32, is_left: bool) -> i32 {
        if let Some(node) = root {
            let node = node.borrow();
            let left = Self::dfs(&node.left, max, true);
            let right = Self::dfs(&node.right, max, false);
            *max = std::cmp::max(*max, std::cmp::max(left, right));
            if is_left {
                right + 1
            } else {
                left + 1
            }
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        assert_eq!(Solution::longest_zig_zag(root), 0);
    }
}
