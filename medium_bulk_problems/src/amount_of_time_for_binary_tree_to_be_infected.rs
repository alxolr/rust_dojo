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
use std::collections::{HashMap, HashSet, VecDeque};
use std::rc::Rc;


// we will traverse the three and transforming it into an undirected graph
fn dfs(
    maybe_prev: &Option<Rc<RefCell<TreeNode>>>,
    maybe_curr: &Option<Rc<RefCell<TreeNode>>>,
    capture: &mut HashMap<i32, Vec<i32>>,
) {
    match maybe_prev {
        Some(prev) => match maybe_curr {
            Some(curr) => {
                let prev_borrow = prev.borrow();
                let curr_borrow = curr.borrow();

                let direct_dir = capture.entry(prev_borrow.val).or_insert(vec![]);
                direct_dir.push(curr_borrow.val);

                let undirect_dir = capture.entry(curr_borrow.val).or_insert(vec![]);
                undirect_dir.push(prev_borrow.val);

                dfs(maybe_curr, &curr_borrow.left, capture);
                dfs(maybe_curr, &curr_borrow.right, capture);
            }
            None => {}
        },
        None => {
            if let Some(curr) = maybe_curr {
                let curr_borrow = curr.borrow();

                dfs(maybe_curr, &curr_borrow.left, capture);
                dfs(maybe_curr, &curr_borrow.right, capture);
            }
        }
    }
}

impl Solution {
    pub fn amount_of_time(root: Option<Rc<RefCell<TreeNode>>>, start: i32) -> i32 {
        let mut graph = HashMap::new();

        // craete the undirected graph
        dfs(&None, &root, &mut graph);

        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        queue.push_back((start, 0));
        let mut max_distance = -1;

        while let Some((node, distance)) = queue.pop_back() {
            max_distance = max_distance.max(distance);

            visited.insert(node);

            if let Some(neighbours) = graph.get(&node) {
                for neighbour in neighbours {
                    if !visited.contains(neighbour) {
                        queue.push_front((*neighbour, distance + 1));
                    }
                }
            }
        }

        max_distance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_ok() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode::new(10)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
            }))),
        })));

        let solution = Solution::amount_of_time(root, 3);

        assert_eq!(solution, 4);
    }
}
