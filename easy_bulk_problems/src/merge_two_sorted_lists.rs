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

fn collect(head: &Box<ListNode>) -> Vec<i32> {
    let mut result = vec![head.val];
    let mut curr = head;
    while let Some(node) = &curr.next {
        result.push(node.val);
        curr = node;
    }

    result
}

struct Solution;

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if list1.is_none() && list2.is_none() {
            return None;
        }

        let result = if list1.is_some() && list2.is_some() {
            let list_one = collect(&list1.unwrap());
            let list_two = collect(&list2.unwrap());

            let mut result = vec![list_one, list_two].concat();

            result.sort_unstable();

            result
        } else if list1.is_some() && list2.is_none() {
            collect(&list1.unwrap())
        } else {
            collect(&list2.unwrap())
        };

        let mut head = ListNode::new(result[0]);
        let mut curr = &mut head;

        for i in 1..result.len() {
            let node = ListNode::new(result[i]);
            curr.next = Some(Box::new(node));
            curr = curr.next.as_mut().unwrap();
        }

        Some(Box::new(head))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_ok() {
        let list1 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { next: None, val: 4 })),
            })),
        }));

        let list2 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode { next: None, val: 4 })),
            })),
        }));

        let result: Option<Box<ListNode>> = Solution::merge_two_lists(list1, list2);
        let expected = vec![1, 1, 2, 3, 4, 4];

        let mut collected_values = vec![];
        if let Some(list) = result {
            collected_values = collect(&list);
        }

        assert_eq!(collected_values, expected);
    }
}
