use std::collections::HashSet;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
impl Solution {
    pub fn modified_list(nums: Vec<i32>, head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let hash_set = nums.into_iter().collect::<HashSet<i32>>();
        let mut head = head;

        // Remove nodes from the beginning
        while let Some(node) = &head {
            if hash_set.contains(&node.val) {
                head = node.next.clone();
            } else {
                break;
            }
        }

        // Remove nodes from the middle/end
        let mut current = &mut head;
        while current.is_some() {
            let node = current.as_mut().unwrap();
            if let Some(next_node) = &node.next {
                if hash_set.contains(&next_node.val) {
                    // Skip the next node by updating the pointer
                    node.next = next_node.next.clone();
                    // Don't advance current, check the new next node
                    continue;
                }
            }
            // Only advance when next node is valid (not deleted) or None
            current = &mut current.as_mut().unwrap().next;
        }

        head
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn modified_list_works_ok() {
        let head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode::new(5))),
                    })),
                })),
            })),
        }));

        let result = Solution::modified_list(vec![1, 2, 3], head);

        assert_eq!(
            result,
            Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode::new(5)))
            }))
        )
    }
}
