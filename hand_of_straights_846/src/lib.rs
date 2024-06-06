use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution;

impl Solution {
    pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
        let group_size = group_size as usize;

        if hand.len() % group_size != 0 {
            return false;
        }

        let mut hand = hand;
        hand.sort_unstable(); // (O(nlogn))

        // Use a binary heap to store the last card of the group as well as the size
        let mut groups_heap: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new();

        // Try to allocate all cards to the existing groups
        for card in hand {
            // Keep a flag to check if the card was assigned to a group
            let mut card_allocated = false;

            // Look through the existing groups and check if it's possible to assign it to any existing
            while let Some(&Reverse((last, size))) = groups_heap.peek() {
                // Check if the group size is not full and the next card is a succession of current
                if last + 1 == card && size < group_size {
                    groups_heap.pop();

                    groups_heap.push(Reverse((card, size + 1)));
                    card_allocated = true;

                    break;
                } else if size == group_size {
                    // if the existing group is already allocated we can just pop it
                    groups_heap.pop();
                } else {
                    break;
                }
            }

            // In case the card was not assigned to any existing groups, start a new group
            if !card_allocated {
                groups_heap.push(Reverse((card, 1)));
            }
        }

        // In the heap main remain non fully allocated groups
        // Check if all the groups in the heap have equal size to the group size
        groups_heap
            .into_iter()
            .all(|Reverse((_, size))| size == group_size as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            ((vec![1, 2, 3, 6, 2, 3, 4, 7, 8], 3), true),
            ((vec![1, 2, 3, 4, 5], 4), false),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((hand, group_size), expected))| {
                let result = Solution::is_n_straight_hand(hand, group_size);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
