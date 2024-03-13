use std::borrow::{Borrow, BorrowMut};

struct Solution;

// Definition for singly-linked list.
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
    pub fn remove_zero_sum_sublists(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let result = Solution::linked_list_to_vec(head);

        Self::vector_to_linked_list(Solution::cleanup_zero_sum(result))
    }

    pub fn cleanup_zero_sum(vec: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = vec.iter().filter(|x| x != &&0).map(|x| *x).collect();

        if result.is_empty() || result.len() == 1 {
            return result;
        }

        let mut left = 0;
        let mut right = result.len() - 1;

        while left < right {
            let sum = result[left..=right].iter().sum::<i32>(); // O(n)

            if sum == 0 {
                result = result
                    .iter()
                    .enumerate()
                    .filter(|(idx, _)| !(left..=right).contains(idx))
                    .map(|(_, x)| *x)
                    .collect();

                return Solution::cleanup_zero_sum(result);
            } else {
                if right > left + 1 {
                    right -= 1;
                } else {
                    left += 1;
                    right = result.len() - 1;
                }
            }
        }

        result
    }

    fn vector_to_linked_list(arr: Vec<i32>) -> Option<Box<ListNode>> {
        if arr.is_empty() {
            return None;
        }

        let first = arr[0];

        let mut head: Option<Box<ListNode>> = Some(Box::new(ListNode {
            val: first,
            next: None,
        }));

        let mut tail = &mut head;

        for val in arr.into_iter().skip(1) {
            let new_node = Some(Box::new(ListNode { val, next: None }));

            if let Some(tail_box) = tail {
                tail_box.as_mut().next = new_node;
                tail = &mut tail_box.next;
            }
        }

        head
    }

    fn linked_list_to_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];
        let mut curr_node = &head;

        while let Some(node) = curr_node {
            let curr_val = node.as_ref().val;

            result.push(curr_val);

            curr_node = &node.as_ref().next;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cleanup_zero_sum() {
        let scenarios = vec![
            (vec![0, 3, -1], vec![3, -1]),
            (vec![0, 1, -1], vec![]),
            (vec![0, 2], vec![2]),
            (vec![0], vec![]),
            (vec![-1, 1], vec![]),
            (vec![1, 2, -3, 3, 1], vec![3, 1]),
            (vec![5, -3, -4, 1, 6, -2, -5], vec![5, -2, -5]),
            (vec![1, 3, 2, -3, -2, 5, 5, -5, 1], vec![1, 5, 1]),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::cleanup_zero_sum(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }

    #[test]
    fn it_works() {
        let head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: -3,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode { val: 1, next: None })),
                    })),
                })),
            })),
        }));

        let expected_head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 1, next: None })),
            })),
        }));

        assert_eq!(Solution::remove_zero_sum_sublists(head), expected_head);
    }
}
