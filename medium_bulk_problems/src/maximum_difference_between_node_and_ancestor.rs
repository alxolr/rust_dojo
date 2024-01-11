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
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        _bfs(&root)
    }
}

fn _bfs(curr: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match curr {
        Some(node) => {
            let mut max = _max_diff(curr, None);

            max = max.max(_bfs(&node.borrow().left));
            max = max.max(_bfs(&node.borrow().right));

            max
        }
        None => i32::MIN,
    }
}

fn _max_diff(curr: &Option<Rc<RefCell<TreeNode>>>, ancestor: Option<i32>) -> i32 {
    match curr {
        Some(node) => match ancestor {
            Some(ancestor) => {
                let mut current = (ancestor - node.borrow().val).abs();

                current = current.max(_max_diff(&node.borrow().left, Some(ancestor)));
                current = current.max(_max_diff(&node.borrow().right, Some(ancestor)));

                current
            }
            None => {
                // this is a root element we should create the route
                let ancestor = node.borrow().val;

                i32::max(
                    _max_diff(&node.borrow().left, Some(ancestor)),
                    _max_diff(&node.borrow().right, Some(ancestor)),
                )
            }
        },
        None => 0, // we passed the leaf level here we have nothing
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_ok() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 3,
                        left: None,
                        right: None,
                    }))),
                    right: None,
                }))),
            }))),
            left: None,
        })));

        let result = Solution::max_ancestor_diff(root);

        assert_eq!(result, 3)
    }
}
