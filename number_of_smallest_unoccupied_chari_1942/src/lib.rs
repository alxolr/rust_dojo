use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution;

impl Solution {
    pub fn smallest_chair(times: Vec<Vec<i32>>, target_friend: i32) -> i32 {
        // Pair each friend's times with their original index
        let mut indexed_times = times
            .iter()
            .enumerate()
            .map(|(index, time)| (time[0], time[1], index))
            .collect::<Vec<_>>();

        // Sort by arrival time
        indexed_times.sort_by(|a, b| a.0.cmp(&b.0));

        // Min-Heap for available seats
        let mut available_seats: BinaryHeap<Reverse<i32>> =
            (0..times.len() as i32).map(Reverse).collect();
        // Min-Heap for occupied seats (end_time, seat_number)
        let mut occupied_seats: BinaryHeap<(i32, i32)> = BinaryHeap::new();

        for (arrival, leaving, index) in indexed_times {
            // Free up seats that are now available
            while let Some(&(end_time, seat)) = occupied_seats.peek() {
                if arrival >= end_time {
                    occupied_seats.pop();
                    available_seats.push(Reverse(seat));
                } else {
                    break;
                }
            }

            // Get the smallest available seat
            let seat = available_seats.pop().unwrap().0;

            // If the current friend is the target friend, return the seat number
            if index as i32 == target_friend {
                return seat;
            }

            // Occupy the seat until the friend's leaving time
            occupied_seats.push((leaving, seat));
        }

        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            ((vec![vec![1, 4], vec![2, 3], vec![4, 6]], 1), 1),
            ((vec![vec![3, 10], vec![1, 5], vec![2, 6]], 0), 2),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((times, target), expected))| {
                let result = Solution::smallest_chair(times, target);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
