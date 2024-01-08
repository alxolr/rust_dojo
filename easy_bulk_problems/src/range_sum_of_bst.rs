pub struct Solution;

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
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        match root {
            Some(node) => {
                let node = node.borrow();
                let value = node.val;

                let mut sum = 0;

                if value > low && value < high {
                    sum += value;
                }

                sum + Solution::range_sum_bst(node.left.clone(), low, high)
                    + Solution::range_sum_bst(node.right.clone(), low, high)
            }

            _ => 0,
        }
    }
}
