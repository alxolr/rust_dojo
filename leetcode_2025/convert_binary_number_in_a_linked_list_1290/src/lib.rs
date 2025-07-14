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
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        // Starting decimal value
        let mut result = 0;
        
        // Traverse the linked list
        let mut current = head;
        while let Some(node) = current {
            // Shift left (multiply by 2) and add current bit
            result = (result << 1) | node.val;
            current = node.next;
        }
        
        result
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let linked_list = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode::new(1))),
            })),
        }));

        assert_eq!(Solution::get_decimal_value(linked_list), 5);
    }
}
