/*
 Find the block wich has the minimal distance towards the requirements
 blocks: Vec<Vec<u8>>, reqs: Vec<u8>

 Example blocks = [[1,0,1],[1,0,0],[0,0,1]]
 [
    1, Gym
    0, School,
    1, Store
 ]

 In the block one we have gym and store

 Req = [1,0,0]
 [
    1, Gym, We value Gym
    0, School, We don't care about School
    0, Store, We don't care about Store
 ]

Algorithm

 define an array of solutions

 traverse each block
    define a solution which will store min distances from current block toward important requirement
    traverse both right and left

 [x] traverse the solutions and find the min one

*/

use std::u32::MAX;

pub fn find_block(blocks: Vec<Vec<u8>>, reqs: Vec<u8>) -> usize {
    let mut solutions: Vec<Vec<u32>> = vec![];
    let block_len = blocks.len();

    for idx in 0..block_len {
        let mut solution_right = vec![(0, false); reqs.len()];

        for block_idx in idx..block_len {
            for req_idx in 0..reqs.len() {
                if reqs[req_idx] == 1 {
                    if blocks[block_idx][req_idx] != 1 && !solution_right[req_idx].1 {
                        solution_right[req_idx].0 += 1;
                    } else {
                        solution_right[req_idx].1 = true;
                    }
                }
            }
        }

        let mut solution_left = vec![(0, false); reqs.len()];

        for block_idx in (0..=idx).rev() {
            for req_idx in 0..reqs.len() {
                if reqs[req_idx] == 1 {
                    if blocks[block_idx][req_idx] != 1  && !solution_left[req_idx].1 {
                        solution_left[req_idx].0 += 1;
                    } else {
                        solution_left[req_idx].1 = true;
                    }
                }
            }
        }

        let solution = solution_right
            .iter()
            .zip(solution_left.iter())
            .map(|(a, b)| {
                if a.1 && b.1 {
                    a.0.min(b.0)
                } else if a.1 {
                    a.0
                } else {
                    b.0
                }
            })
            .collect::<Vec<_>>();

        solutions.push(solution);
    }

    find_solution(solutions)
}

pub fn find_solution(solutions: Vec<Vec<u32>>) -> usize {
    let mut solution_index = 0;
    let mut solution_min = MAX;

    for (index, solution) in solutions.iter().enumerate() {
        let min = solution.iter().max().unwrap();
        if solution_min > *min {
            solution_min = *min;
            solution_index = index;
        }
    }

    solution_index
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_min_solution() {
        let solutions = vec![
            vec![1, 0, 4],
            vec![0, 1, 3],
            vec![0, 0, 2],
            vec![1, 0, 1],
            vec![2, 0, 0],
        ];

        let expected = 3;
        assert_eq!(find_solution(solutions), expected);
    }

    #[test]
    fn it_finds_the_right_block() {
        let blocks = vec![
            vec![0, 1, 0],
            vec![1, 0, 0],
            vec![1, 1, 0],
            vec![0, 1, 0],
            vec![0, 1, 1],
        ];
        let reqs = vec![1, 1, 1];
        let expected_result = 3;

        assert_eq!(find_block(blocks, reqs), expected_result);
    }
}
