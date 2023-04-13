pub struct Solution;

impl Solution {
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let mut stack = Vec::new();
        let mut pushed = pushed.into_iter();
        let mut popped = popped.into_iter();

        while let Some(popped) = popped.next() {
            while stack.last() != Some(&popped) {
                if let Some(pushed) = pushed.next() {
                    stack.push(pushed);
                } else {
                    return false;
                }
            }
            stack.pop();
        }

        true
    }
}


// The idea is to use a stack to simulate the push and pop operations.
// Iterate through the popped array, and for each element, we first check if it is the same as the top of the stack.
// If so, we just pop it out of the stack.
// If not, we push elements from the pushed array into the stack until we either push the target element into the stack or we have pushed all elements in the pushed array.


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            ((vec![1, 2, 3, 4, 5], vec![4, 5, 3, 2, 1]), true),
            ((vec![1, 2, 3, 4, 5], vec![4, 3, 5, 1, 2]), false),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((pushed, popped), expected))| {
                let result = Solution::validate_stack_sequences(pushed, popped);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
