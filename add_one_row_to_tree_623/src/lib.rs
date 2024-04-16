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
    fn dfs(root: &mut Option<Rc<RefCell<TreeNode>>>, val: i32, curr_depth: i32, target_depth: i32) {
        match root {
            Some(node) => {
                let mut node = node.as_ref().borrow_mut();
                if curr_depth == target_depth - 1 {
                    let mut new_node = TreeNode::new(val);
                    let left_node: Option<Rc<RefCell<TreeNode>>> = node.left.take();
                    new_node.left = left_node;
                    node.left = Some(Rc::new(RefCell::new(new_node)));

                    let mut new_node = TreeNode::new(val);
                    let right_node: Option<Rc<RefCell<TreeNode>>> = node.right.take();
                    new_node.right = right_node;
                    node.right = Some(Rc::new(RefCell::new(new_node)));
                } else {
                    Self::dfs(&mut node.left, val, curr_depth + 1, target_depth);
                    Self::dfs(&mut node.right, val, curr_depth + 1, target_depth);
                }
            }
            None => {}
        }
    }

    pub fn add_one_row(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
        depth: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut root = root;

        match depth {
            1 => {
                let mut new_node = TreeNode::new(val);
                new_node.left = root;

                Some(Rc::new(RefCell::new(new_node)))
            }
            _ => {
                Self::dfs(&mut root, val, 1, depth);

                root
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
                right: None,
            }))),
        })));

        let expected_root = Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                }))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 6,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
                    right: None,
                }))),
            }))),
        })));

        assert_eq!(Solution::add_one_row(root, 1, 2), expected_root);
    }
}
