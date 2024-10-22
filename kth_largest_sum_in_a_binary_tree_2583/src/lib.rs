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

pub struct Solution;

impl Solution {
    fn dfs(i: usize, root: &Option<Rc<RefCell<TreeNode>>>, sums: &mut Vec<i64>) {
        if let Some(node_ref) = root {
            let node = node_ref.as_ref().borrow();
            sums[i] += node.val as i64;

            Solution::dfs(i + 1, &node.left, sums);
            Solution::dfs(i + 1, &node.right, sums);
        }
    }

    pub fn kth_largest_level_sum(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i64 {
        let mut sums = vec![0; 15];
        Solution::dfs(0, &root, &mut sums);

        sums.sort();

        *sums.iter()
            .nth_back((k - 1) as usize)
            .and_then(|x| if x > &0 { Some(x) } else { None })
            .unwrap_or(&-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // [5,8,9,2,1,3,7,4,6]
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 8,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        assert_eq!(Solution::kth_largest_level_sum(root, 2), 9);
    }
}
