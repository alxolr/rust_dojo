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
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut queue = VecDeque::new();

        queue.push_back((1, root.unwrap()));
        let mut level_sums = std::collections::HashMap::new();

        while let Some((level, node)) = queue.pop_front() {
            let node_ref = node.borrow();
            *level_sums.entry(level).or_insert(0) += node_ref.val;

            if let Some(child) = &node_ref.left {
                queue.push_back((level + 1, Rc::clone(child)));
            }

            if let Some(child) = &node_ref.right {
                queue.push_back((level + 1, Rc::clone(child)));
            }
        }

        let mut max_sum = i32::MIN;
        let mut max_level = 1;

        for (&level, &sum) in level_sums.iter() {
            if sum > max_sum {
                max_sum = sum;
                max_level = level;
            }
        }

        max_level as i32
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            (
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 7,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                        right: Some(Rc::new(RefCell::new(TreeNode::new(-8)))),
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
                }))),
                2,
            ),
            (
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None,
                }))),
                1,
            ),
            (
                Some(Rc::new(RefCell::new(TreeNode {
                    val: -100,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: -200,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(-20)))),
                        right: Some(Rc::new(RefCell::new(TreeNode::new(-5)))),
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: -300,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(-10)))),
                        right: None,
                    }))),
                }))),
                3,
            ),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::max_level_sum(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
