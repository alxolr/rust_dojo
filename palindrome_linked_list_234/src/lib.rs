use std::borrow::Borrow;

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
    fn is_palindrome_vec(arr: &Vec<i32>) -> bool {
        let mut start = 0;
        let mut end = arr.len() - 1;

        while start < end {
            if arr[start] != arr[end] {
                return false;
            }

            start += 1;
            end -= 1;
        }

        true
    }
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut values = vec![];

        let mut current = &head;

        while let Some(current_node) = current {
            let borrowed = current_node.as_ref();
            let value = borrowed.val;
            values.push(value);

            current = &borrowed.next;
        }

        Solution::is_palindrome_vec(&values)
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
