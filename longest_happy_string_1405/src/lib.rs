use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
        // Create an iterator of letters and counts to store in the heap
        let letters = vec![(a, 'a'), (b, 'b'), (c, 'c')]
            .into_iter()
            .filter(|(count, _)| *count > 0);

        let mut heap = BinaryHeap::from_iter(letters);
        let mut result = String::with_capacity((a + b + c) as usize);

        let mut total = a + b + c;
        let mut last = None;

        while let Some((mut count, ch)) = heap.pop() {
            // Add the current character to the string
            result.push(ch);
            // Decrease the count and total
            count -= 1;
            total -= 1;

            // If the current character's count is more than twice the remaining characters,
            // add it one more time to ensure we don't exceed the "at most two consecutive" rule
            if count > (total - count) * 2 {
                count -= 1;
                total -= 1;
                result.push(ch);
            }

            // If there was a last character stored, push it back to the heap
            if let Some(last) = last.take() {
                heap.push(last);
            }

            // If the current character still has a positive count, store it as the last character
            if count > 0 {
                last = Some((count, ch));
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            ((1, 1, 7), "ccbccacc".to_string()),
            ((7, 1, 0), "aabaa".to_string()),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((a, b, c), expected))| {
                let result = Solution::longest_diverse_string(a, b, c);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
