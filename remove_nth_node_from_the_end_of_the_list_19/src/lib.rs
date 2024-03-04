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

struct Solution;

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut curr_ptr = &head;

        let mut len = 0;
        while let Some(curr) = curr_ptr {
            len += 1;
            curr_ptr = &curr.next;
        }

        let pos = len - n;

        if pos == 0 {
            return head.unwrap().next;
        }

        let mut head = head;
        let mut curr_ptr = &mut head;
        let mut i = 0;

        while let Some(curr) = curr_ptr {
            if i == pos - 1 {
                let next = curr.next.take();
                curr.next = next.unwrap().next;
                break;
            }
            i += 1;
            curr_ptr = &mut curr.next;
        }

        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            (
                (
                    Some(Box::new(ListNode {
                        val: 1,
                        next: Some(Box::new(ListNode {
                            val: 2,
                            next: Some(Box::new(ListNode {
                                val: 3,
                                next: Some(Box::new(ListNode {
                                    val: 4,
                                    next: Some(Box::new(ListNode { val: 5, next: None })),
                                })),
                            })),
                        })),
                    })),
                    2,
                ),
                Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode {
                            val: 3,
                            next: Some(Box::new(ListNode { val: 5, next: None })),
                        })),
                    })),
                })),
            ),
            ((Some(Box::new(ListNode { val: 1, next: None })), 1), None),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((root, n), expected))| {
                let result = Solution::remove_nth_from_end(root, n);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
