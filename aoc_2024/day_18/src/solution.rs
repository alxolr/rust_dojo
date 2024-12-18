use crate::error::Result;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
pub struct Solution;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: i32,
    position: (usize, usize),
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn part_1(input: &str, rows: usize, cols: usize, bytes: usize) -> Result<i32> {
        let mut grid = vec![vec!['.'; cols]; rows];
        let coordinates = load_coordinates(input, bytes);

        for (row, col) in coordinates {
            grid[row][col] = '#';
        }

        if let Some(value) = shortest_path(rows, cols, &grid) {
            return Ok(value);
        } else {
            panic!("No solution")
        }
    }

    pub fn part_2(input: &str, rows: usize, cols: usize, bytes: usize) -> Result<String> {
        let mut grid = vec![vec!['.'; cols]; rows];

        let (coordinates, next_coordinates) = load_coordinates_part_2(input, bytes);
        for (row, col) in coordinates {
            grid[row][col] = '#';
        }

        for (new_row, new_col) in next_coordinates {
            grid[new_row][new_col] = '#'; // place the new coordinate

            if let Some(_) = shortest_path(rows, cols, &grid) {
                continue;
            } else {
                return Ok(format!("{},{}", new_col, new_row));
            }
        }

        Ok("".to_string())
    }
}

fn shortest_path(rows: usize, cols: usize, grid: &Vec<Vec<char>>) -> Option<i32> {
    let start = (0, 0);
    let goal = (rows - 1, cols - 1);
    let mut dist: Vec<Vec<i32>> = vec![vec![i32::MAX; cols]; rows];
    let mut heap = BinaryHeap::new();
    dist[start.0][start.1] = 0;
    heap.push(State {
        cost: 0,
        position: start,
    });
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    while let Some(State { cost, position }) = heap.pop() {
        if position == goal {
            return Some(cost);
        }

        if cost > dist[position.0][position.1] {
            continue;
        }

        for direction in &directions {
            let new_position = (
                (position.0 as isize + direction.0) as usize,
                (position.1 as isize + direction.1) as usize,
            );

            if new_position.0 < rows
                && new_position.1 < cols
                && grid[new_position.0][new_position.1] != '#'
            {
                let next_cost = cost + 1;

                if next_cost < dist[new_position.0][new_position.1] {
                    dist[new_position.0][new_position.1] = next_cost;
                    heap.push(State {
                        cost: next_cost,
                        position: new_position,
                    });
                }
            }
        }
    }

    None
}

fn load_coordinates(input: &str, take: usize) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|line| {
            let mut coords = line.split(",");
            let row = coords.next().unwrap().parse::<usize>().unwrap();
            let col = coords.next().unwrap().parse::<usize>().unwrap();
            (col, row)
        })
        .take(take)
        .collect()
}

fn load_coordinates_part_2(input: &str, take: usize) -> (Vec<(usize, usize)>, Vec<(usize, usize)>) {
    let coordinates = input
        .lines()
        .map(|line| {
            let mut coords = line.split(",");
            let row = coords.next().unwrap().parse::<usize>().unwrap();
            let col = coords.next().unwrap().parse::<usize>().unwrap();
            (col, row)
        })
        .collect::<Vec<(usize, usize)>>();

    let (left, right) = coordinates.split_at(take);

    (left.to_vec(), right.to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() -> Result<()> {
        let input = r#"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0"#;
        assert_eq!(Solution::part_1(input, 7, 7, 12)?, 22);

        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let input = r#"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0"#;
        assert_eq!(Solution::part_2(input, 7, 7, 12)?, "6,1".to_string());

        Ok(())
    }
}
