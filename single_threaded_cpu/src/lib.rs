use std::{
    cmp::{Ordering, Reverse},
    collections::BinaryHeap,
};

struct Solution;

impl Solution {
    pub fn get_order(tasks: Vec<Vec<i32>>) -> Vec<i32> {
        let mut priority_process: BinaryHeap<Reverse<ProcessTask>> = BinaryHeap::new();
        let mut tasks = into_tasks(tasks);
        let mut result = vec![];
        let mut timer = tasks.peek().unwrap().0.enqueue_time;
        let mut closest_task_timer = 0;

        while tasks.len() + priority_process.len() > 0 {
            let mut buffer = vec![];

            while let Some(task_peek) = tasks.peek() {
                if task_peek.0.enqueue_time > timer {
                    closest_task_timer = task_peek.0.enqueue_time;
                    break;
                }

                if let Some(task) = tasks.pop() {
                    buffer.push(task.0);
                }
            }

            buffer
                .into_iter()
                .map(|task| task.into())
                .for_each(|task: ProcessTask| priority_process.push(Reverse(task)));

            if !priority_process.is_empty() {
                let task = priority_process.pop().unwrap();
                result.push(task.0.id as i32);
                timer += task.0.process_time;
            } else {
                timer = closest_task_timer;
            }
        }

        result
    }
}

fn into_tasks(tasks: Vec<Vec<i32>>) -> BinaryHeap<Reverse<EnqueTask>> {
    tasks
        .into_iter()
        .enumerate()
        .map(|(id, item)| EnqueTask {
            id,
            enqueue_time: item[0],
            process_time: item[1],
        })
        .map(|task| Reverse(task))
        .fold(BinaryHeap::new(), |mut acc, item| {
            acc.push(item);
            acc
        })
}

#[derive(Eq, Ord)]
pub struct EnqueTask {
    id: usize,
    enqueue_time: i32,
    process_time: i32,
}

impl Into<ProcessTask> for EnqueTask {
    fn into(self) -> ProcessTask {
        ProcessTask {
            id: self.id,
            enqueue_time: self.enqueue_time,
            process_time: self.process_time,
        }
    }
}

impl PartialEq for EnqueTask {
    fn eq(&self, other: &Self) -> bool {
        self.enqueue_time == other.enqueue_time
    }
}

impl PartialOrd for EnqueTask {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.enqueue_time == other.enqueue_time {
            self.id.partial_cmp(&other.id)
        } else {
            self.enqueue_time.partial_cmp(&other.enqueue_time)
        }
    }
}

#[derive(Eq, Ord)]
pub struct ProcessTask {
    id: usize,
    enqueue_time: i32,
    process_time: i32,
}

impl PartialEq for ProcessTask {
    fn eq(&self, other: &Self) -> bool {
        self.process_time == other.process_time
    }
}

impl PartialOrd for ProcessTask {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.process_time == other.process_time {
            self.id.partial_cmp(&other.id)
        } else {
            self.process_time.partial_cmp(&other.process_time)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scenario_1() {
        let tasks = vec![vec![1, 2], vec![2, 4], vec![3, 2], vec![4, 1]];
        let result = Solution::get_order(tasks);
        assert_eq!(result, vec![0, 2, 3, 1]);
    }

    #[test]
    fn scenario_two() {
        let tasks = vec![vec![7, 10], vec![7, 12], vec![7, 5], vec![7, 4], vec![7, 2]];
        let result = Solution::get_order(tasks);
        assert_eq!(result, vec![4, 3, 2, 0, 1]);
    }

    #[test]
    fn scenario_three() {
        let tasks = vec![
            vec![19, 13],
            vec![16, 9],
            vec![21, 10],
            vec![32, 25],
            vec![37, 4],
            vec![49, 24],
            vec![2, 15],
            vec![38, 41],
            vec![37, 34],
            vec![33, 6],
            vec![45, 4],
            vec![18, 18],
            vec![46, 39],
            vec![12, 24],
        ];
        let result = Solution::get_order(tasks);
        assert_eq!(result, vec![6, 1, 2, 9, 4, 10, 0, 11, 5, 13, 3, 8, 12, 7]);
    }

    #[test]
    fn scenario_four() {
        let tasks = vec![
            vec![23, 40],
            vec![10, 32],
            vec![12, 18],
            vec![10, 39],
            vec![25, 4],
            vec![18, 12],
            vec![38, 18],
            vec![36, 1],
            vec![26, 5],
            vec![45, 35],
            vec![5, 43],
            vec![19, 19],
            vec![46, 41],
            vec![25, 22],
            vec![29, 17],
            vec![26, 33],
            vec![49, 45],
            vec![43, 44],
            vec![50, 2],
        ];
        let result = Solution::get_order(tasks);
        assert_eq!(
            result,
            vec![10, 7, 4, 18, 8, 5, 14, 2, 6, 11, 13, 1, 15, 9, 3, 0, 12, 17, 16]
        );
    }

    #[test]
    fn test_discontinuity() {
        let tasks = vec![
            vec![5, 2],
            vec![7, 2],
            vec![9, 4],
            vec![6, 3],
            vec![5, 10],
            vec![1, 1],
        ];
        let result = Solution::get_order(tasks);

        assert_eq!(result, vec![5, 0, 1, 3, 2, 4]);
    }

    #[test]
    fn test_discontinuity_big_timer() {
        let tasks = vec![vec![100, 100], vec![1000000000, 1000000000]];
        let result = Solution::get_order(tasks);

        assert_eq!(result, vec![0, 1]);
    }
}
