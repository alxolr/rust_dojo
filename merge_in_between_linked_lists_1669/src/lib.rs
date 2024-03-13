pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> ListNode {
        ListNode { next: None, val }
    }
}

struct Solution;

impl Solution {
    pub fn merge_in_between(
        list1: Option<Box<ListNode>>,
        a: i32,
        b: i32,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        dummy.next = list1;

        let mut curr = &mut dummy;

        for _ in 0..a {
            curr = curr.next.as_mut().unwrap()
        }

        let mut after = &mut curr.next;

        for _ in a..=b {
            after = &mut after.as_mut().unwrap().next
        }

        let after_b = after.take(); // Detach the rest of the list after `b`, this will allow the next line for the borrow checker
        curr.next = list2;

        while let Some(ref mut next) = curr.next {
            curr = next;
        }
        
        curr.next = after_b;
        dummy.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
