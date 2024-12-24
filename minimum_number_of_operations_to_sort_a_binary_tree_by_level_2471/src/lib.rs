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
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;
impl Solution {
    pub fn minimum_operations(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut queue = VecDeque::new();
        let mut levels = HashMap::new();

        if let Some(root) = root {
            queue.push_back((root, 0));
        }

        while let Some((node, level)) = queue.pop_front() {
            let node = node.as_ref().borrow();

            levels.entry(level).or_insert(vec![]).push(node.val);
            if let Some(left) = &node.left {
                queue.push_back((left.clone(), level + 1));
            }

            if let Some(right) = &node.right {
                queue.push_back((right.clone(), level + 1));
            }
        }

        let min_swaps = levels.iter().map(|(_, values)| min_swaps(values)).sum();

        min_swaps
    }
}

fn min_swaps(nums: &Vec<i32>) -> i32 {
    let mut nums = nums.clone();
    let mut target = nums.clone();
    target.sort();

    let mut swaps = 0;

    for id in 0..nums.len() {
        if target[id] != nums[id] {
            // we need to swap
            let pos = target.iter().position(|x| *x == nums[id]).unwrap();
            let temp = nums[pos];
            nums[pos] = nums[id];
            nums[id] = temp;

            swaps += 1;
        }
    }

    swaps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
        })));

        let min_operations = Solution::minimum_operations(root);
        assert_eq!(min_operations, 1)
    }
}
