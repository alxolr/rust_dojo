struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap};

#[derive(PartialEq, Eq)]
struct Color(char, i32); // We will store the char and the cost

impl Ord for Color {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.1.cmp(&self.1)
    }
}
impl PartialOrd for Color {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.1.partial_cmp(&other.1)
    }
}


impl Solution {
    pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
        let mut buffer = BinaryHeap::new();
        let mut min_time = 0;

        for (idx, ch) in colors.chars().enumerate() {
            if buffer.is_empty() {
                buffer.push(Reverse(Color(ch, needed_time[idx])));
            } else {
                if let Some(color) = buffer.peek() {
                    if color.0 .0 == ch {
                        buffer.push(Reverse(Color(ch, needed_time[idx])))
                    } else {
                        // we found a divergent color we need to pop everything from the stack till only one color remains
                        // and then we need to add the new color in the stack
                        min_time += drain(&mut buffer);

                        // start a new cycle with the new color
                        buffer.push(Reverse(Color(ch, needed_time[idx])))
                    }
                }
            }
        }

        // for the case when we have multiple colors in the end
        // we need to flush the buffer
        min_time += drain(&mut buffer);

        min_time
    }
}

fn drain(buffer: &mut BinaryHeap<Reverse<Color>>) -> i32 {
    let mut cost = 0;

    while buffer.len() > 1 {
        cost += buffer.pop().unwrap().0 .1;
    }

    buffer.clear();

    cost
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_cost_ok() {
        let scenarios = vec![
            (("abaac".to_string(), vec![1, 2, 3, 4, 5]), 3),
            (("abc".to_string(), vec![1, 2, 3]), 0),
            (("aabaa".to_string(), vec![1, 2, 3, 4, 1]), 2),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((colors, needed_time), expected))| {
                let result = Solution::min_cost(colors, needed_time);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
