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
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    fn bfs(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
        let mut queue = VecDeque::new();
        let mut traversed = vec![];

        queue.push_back(root);

        while !queue.is_empty() {
            let node = queue.pop_front().unwrap();

            if let Some(node) = node {
                let node = node.borrow();
                traversed.push(Some(node.val));

                queue.push_back(node.left.clone());
                queue.push_back(node.right.clone());
            } else {
                traversed.push(None);
            }
        }

        traversed
    }

    pub fn is_even_odd_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let traversed = Self::bfs(root);
        let traversed_len = traversed.len();
        let height = (traversed_len as f64).log2() as usize;

        for level in 0..height {
            let range = 2_usize.pow(level as u32)..2_usize.pow(level as u32 + 1);

            if level % 2 == 0 {
                if !Self::increasing(&traversed[range]) {
                    return false;
                }
            } else {
                if !Self::decreasing(&traversed[range]) {
                    return false;
                }
            }
        }

        true
    }

    fn increasing(arr: &[Option<i32>]) -> bool {
        let mut prev = None;

        for &val in arr {
            if let Some(val) = val {
                if prev.is_none() {
                    prev = Some(val);
                } else {
                    if val <= prev.unwrap() {
                        return false;
                    }
                    prev = Some(val);
                }
            }
        }

        true
    }

    fn decreasing(arr: &[Option<i32>]) -> bool {
        let mut prev = None;

        for &val in arr {
            if let Some(val) = val {
                if prev.is_none() {
                    prev = Some(val);
                } else {
                    if val >= prev.unwrap() {
                        return false;
                    }
                    prev = Some(val);
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
