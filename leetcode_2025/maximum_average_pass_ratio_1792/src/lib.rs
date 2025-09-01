use std::collections::BinaryHeap;

#[derive(Debug)]
pub struct Class {
    gain: f64,
    pass: i32,
    total: i32,
}

impl Class {
    fn new(pass: i32, total: i32) -> Self {
        let current_ratio = pass as f64 / total as f64;
        let new_ratio = (pass + 1) as f64 / (total + 1) as f64;
        let potential_gain = new_ratio - current_ratio;

        Class { gain: potential_gain, pass, total }
    }

    fn add_student(&self) -> Class {
        Class::new(self.pass + 1, self.total + 1)
    }

    fn ratio(&self) -> f64 {
        self.pass as f64 / self.total as f64
    }
}

impl Ord for Class {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.gain.total_cmp(&other.gain)
    }
}

impl PartialEq for Class {
    fn eq(&self, other: &Self) -> bool {
        self.gain == other.gain
    }
}

impl Eq for Class {}

impl PartialOrd for Class {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.gain.partial_cmp(&other.gain)
    }
}

impl Solution {
    pub fn max_average_ratio(classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
        let mut heap = classes
            .into_iter()
            .map(|x| Class::new(x[0], x[1]))
            .collect::<BinaryHeap<_>>();

        for _ in 0..extra_students {
            if let Some(class) = heap.pop() {
                let new_class = class.add_student();
                heap.push(new_class);
            }
        }

        let count = heap.len() as f64;
        let sum: f64 = heap.iter().map(|x| x.ratio()).sum();

        sum / count
    }
}

pub struct Solution;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            (
                (vec![vec![1, 2], vec![3, 5], vec![2, 2]], 2),
                0.7833333333333333,
            ),
            (
                (vec![vec![2, 4], vec![3, 9], vec![4, 5], vec![2, 10]], 4),
                0.53485,
            ),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::max_average_ratio(input.0, input.1);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
