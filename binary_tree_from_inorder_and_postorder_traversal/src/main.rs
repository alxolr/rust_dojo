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

struct Solution;

impl Solution {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if inorder.is_empty() {
            return None;
        }

        let mut root = TreeNode::new(postorder[postorder.len() - 1]);
        let mut stack = vec![Rc::new(RefCell::new(root))];
        let mut inorder_index = inorder.len() - 1;

        for i in (0..postorder.len()).rev() {
            let mut node = stack.pop().unwrap();
            if node.borrow().val != inorder[inorder_index] {
                node.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(postorder[i]))));
                stack.push(node);
                stack.push(node.clone().borrow().right.clone().unwrap());
            } else {
                inorder_index -= 1;
                while !stack.is_empty()
                    && stack[stack.len() - 1].borrow().val == inorder[inorder_index]
                {
                    node = stack.pop().unwrap();
                    inorder_index -= 1;
                }
                node.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(postorder[i]))));
                stack.push(node.borrow().left.clone().unwrap());
            }
        }

        Some(stack.pop().unwrap())
    }
}
