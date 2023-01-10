// Definition for a binary tree node.
struct Solution;

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

fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
    let mut result = vec![];
    let mut stack = vec![root];

    while !stack.is_empty() {
        let curr = stack.pop().unwrap();

        if let Some(node) = curr {
            result.push(Some(node.borrow().val));
            stack.push(node.borrow().right.clone());
            stack.push(node.borrow().left.clone());
        } else {
            result.push(None);
        }
    }

    result
}

impl Solution {
    pub fn is_same_tree(
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let left_values = preorder_traversal(left);
        let right_values = preorder_traversal(right);

        left_values
            .iter()
            .zip(right_values.iter())
            .all(|(a, b)| a == b)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![(
            (
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 3,
                            left: None,
                            right: None,
                        }))),
                        right: None,
                    }))),
                }))),
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 3,
                            left: None,
                            right: None,
                        }))),
                        right: None,
                    }))),
                }))),
            ),
            true,
        )];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((left, right), expected))| {
                let result = Solution::is_same_tree(left, right);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
