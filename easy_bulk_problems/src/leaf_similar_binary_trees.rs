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
use std::collections::VecDeque;
use std::rc::Rc;

fn capture_leafs(node: &Option<Rc<RefCell<TreeNode>>>, leafs: &mut VecDeque<i32>) {
    if let Some(node) = node {
        let bor_node = node.borrow();

        if bor_node.left.is_none() && bor_node.right.is_none() {
            leafs.push_back(bor_node.val);
        } else {
            capture_leafs(&bor_node.left, leafs);
            capture_leafs(&bor_node.right, leafs);
        }
    }
}

impl Solution {
    pub fn leaf_similar(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let mut leafs1 = VecDeque::new();
        let mut leafs2 = VecDeque::new();

        capture_leafs(&root1, &mut leafs1);
        capture_leafs(&root2, &mut leafs2);

        leafs1 == leafs2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_ok() {
        // root1 = [3,5,1,6,7,4,2,null,null,null,null,null,null,9,11,null,null,8,10]

        let root1: TreeNode = TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 6,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 11,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(8)))),
                        right: Some(Rc::new(RefCell::new(TreeNode::new(10)))),
                    }))),
                }))),
            }))),
        };

        // root2 = [3,5,1,6,2,9,8,null,null,7,4]
        let root2 = TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 6,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 9,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 8,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                }))),
            }))),
        };

        assert_eq!(
            Solution::leaf_similar(
                Some(Rc::new(RefCell::new(root1))),
                Some(Rc::new(RefCell::new(root2)))
            ),
            false
        );
    }
}
