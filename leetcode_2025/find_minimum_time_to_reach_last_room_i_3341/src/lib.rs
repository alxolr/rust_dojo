use std::{cmp::Reverse, collections::BinaryHeap};

/// Observations:
/// We have nxm matrix, and in essence we need to find the shortest path, so for this an algorithm like
/// dijkstra can do the trick.
///
/// But the main problem is that we can't move right away to adjacent neighboring tiles as there is a minimum timer
/// we need to wait.
///
/// So in essence, we will have a current score, if the score is less than the tile we want to visit than the score
/// will become the tile we intend on visiting, all the other things are dijkstra algorithm.
///
/// There are multiple ways to implement Dijkstra but a more efficient algorithm will be using a Min Binary Heap.
///
/// Notes on how to implement Dijkstra as I remember it
/// 1. Create a cost matrix if we want to minimize then fill it with i32::MAX
/// 2. Initialize the starting position in our case [0,0] = 0;
/// 3. Create the a finish position (n, m);
///
/// Add in the binary heap, the (cost, new_pos)

#[derive(PartialEq, Eq)]
struct Item(i32, (isize, isize));

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl Solution {
    pub fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
        let rows = move_time.len();
        let cols = move_time[0].len();

        let mut costs = vec![vec![i32::MAX; cols]; rows];
        costs[0][0] = 0;

        let mut queue: BinaryHeap<Reverse<Item>> = BinaryHeap::new();

        queue.push(Reverse(Item(0, (0, 0))));

        let deltas = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        let is_valid_point = |n_row: isize, n_col: isize| -> bool {
            n_row >= 0 && n_row < rows as isize && n_col >= 0 && n_col < cols as isize
        };

        while let Some(item) = queue.pop() {
            let Reverse(Item(cost, (row, col))) = item;

            for (d_row, d_col) in deltas {
                let new_row = row + d_row;
                let new_col = col + d_col;

                if is_valid_point(new_row, new_col) {
                    let n_row = new_row as usize;
                    let n_col = new_col as usize;
                    let new_cost = if cost >= move_time[n_row][n_col] {
                        cost + 1
                    } else {
                        move_time[n_row][n_col] + 1
                    };

                    if new_cost < costs[n_row][n_col] {
                        costs[n_row][n_col] = new_cost;
                        queue.push(Reverse(Item(new_cost, (n_row as isize, n_col as isize))));
                    }
                }
            }
        }

        costs[rows - 1][cols - 1]
    }
}

pub struct Solution;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            (vec![vec![0, 4], vec![4, 4]], 6),
            (vec![vec![0, 0, 0], vec![0, 0, 0]], 3),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::min_time_to_reach(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
