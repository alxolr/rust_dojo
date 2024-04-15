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
    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, stack: &Vec<i32>, result: &mut Vec<i32>) {
        match root {
            Some(node) => {
                let borrow = node.as_ref().borrow();

                let mut stack = stack.clone();
                stack.push(borrow.val);

                if borrow.left.is_none() && borrow.right.is_none() {
                    let leaf_num = stack.iter().map(|x| x.to_string()).collect::<String>();
                    let leaf_num = leaf_num.parse::<i32>().unwrap_or(0);

                    result.push(leaf_num);
                    stack.pop();
                } else {
                    Self::dfs(&borrow.left, &stack, result);
                    Self::dfs(&borrow.right, &stack, result);
                }
            }
            None => {}
        }
    }

    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = vec![];

        Self::dfs(&root, &vec![], &mut ans);

        ans.iter().sum()
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
                val: 9,
                left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
        })));

        assert_eq!(Solution::sum_numbers(root), 1026);
    }

    #[test]
    fn it_works_with_bigger() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        })));

        assert_eq!(Solution::sum_numbers(root), 25);
    }
}
