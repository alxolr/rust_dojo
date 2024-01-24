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
    pub fn pseudo_palindromic_paths(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::dfs(&root, vec![0; 10])
    }

    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, mut counter: Vec<usize>) -> i32 {
        match node {
            Some(node) => {
                let curr_node = node.borrow();
                counter[curr_node.val as usize] += 1;

                if curr_node.left.is_none() && curr_node.right.is_none() {
                    if Solution::is_palindrom(&counter) {
                        return 1;
                    } else {
                        return 0;
                    }
                }

                return Solution::dfs(&curr_node.left, counter.clone())
                    + Solution::dfs(&curr_node.right, counter.clone());
            }
            None => 0,
        }
    }

    fn is_palindrom(arr: &Vec<usize>) -> bool {
        let frequency = arr.iter().filter(|x| **x % 2 != 0).count();

        frequency <= 1
    }
}

#[cfg(test)]
mod tests {
    use std::cell::Ref;

    use super::*;

    #[test]
    fn test_solution_ok() {
        let scenarios = vec![
            // 2, 3,1, 3,1, null,1
            (
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 3,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 3,
                            left: None,
                            right: None,
                        }))),
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 1,
                            left: None,
                            right: None,
                        }))),
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1,
                        left: None,
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 1,
                            left: None,
                            right: None,
                        }))),
                    }))),
                }))),
                2,
            ),
            // 2,1,1, 1,3,null,null,null,null,null,1
            (
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))), // m
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 3,
                            left: None,
                            right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                        }))), // m
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                }))),
                1,
            ),

             // 9
             (
                Some(Rc::new(RefCell::new(TreeNode::new(9)))),
                1,
            ),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::pseudo_palindromic_paths(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
