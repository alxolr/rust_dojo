type FlaggedSolution = Vec<(u32, bool)>;

pub fn optimize_blocks(blocks: Vec<Vec<u8>>, reqs: Vec<u8>) -> usize {
    let mut solutions: Vec<Vec<u32>> = vec![];

    for idx in 0..blocks.len() {
        let solution = compute_block_solution(idx, &blocks, &reqs);

        solutions.push(solution);
    }

    find_min_solution(solutions)
}

fn compute_block_solution(block_idx: usize, blocks: &Vec<Vec<u8>>, reqs: &[u8]) -> Vec<u32> {
    let mut right_solution: FlaggedSolution = vec![(0, false); reqs.len()];
    let mut left_solution: FlaggedSolution = vec![(0, false); reqs.len()];

    for local_idx in block_idx..blocks.len() {
        compute_requirement_solution(reqs, blocks, local_idx, &mut right_solution);
    }

    for local_idx in (0..block_idx).rev() {
        compute_requirement_solution(reqs, blocks, local_idx, &mut left_solution);
    }

    find_min_local_solution(left_solution, right_solution)
}

fn find_min_solution(solutions: Vec<Vec<u32>>) -> usize {
    let mut min_position = (u32::MAX, 0);

    for (idx, solution) in solutions.iter().enumerate() {
        let local_max = solution.iter().max().unwrap();

        if local_max < &min_position.0 {
            min_position.0 = *local_max;
            min_position.1 = idx;
        }
    }

    min_position.1
}

fn compute_requirement_solution(
    reqs: &[u8],
    blocks: &[Vec<u8>],
    local_idx: usize,
    solution: &mut FlaggedSolution,
) {
    for (req_idx, req) in reqs.iter().enumerate() {
        if req == &1 {
            if blocks[local_idx][req_idx] == 1 {
                solution[req_idx].1 = true;
            } else if !solution[req_idx].1 {
                solution[req_idx].0 += 1;
            }
        }
    }
}

fn find_min_local_solution(
    left_solution: FlaggedSolution,
    right_solution: FlaggedSolution,
) -> Vec<u32> {
    left_solution
        .iter()
        .zip(right_solution.iter())
        .map(|(left, right)| {
            if left.1 && right.1 {
                left.0.min(right.0)
            } else if left.1 {
                left.0
            } else {
                right.0
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optmize_blocks() {
        let reqs = vec![1, 1, 1];
        let blocks = vec![
            vec![0, 1, 0],
            vec![1, 0, 0],
            vec![1, 1, 0],
            vec![0, 1, 0],
            vec![0, 1, 1],
        ];

        assert_eq!(optimize_blocks(blocks, reqs), 3);
    }

    #[test]
    fn test_find_min_solution() {
        let solutions = vec![vec![1, 0, 4], vec![0, 0, 5], vec![0, 0, 3], vec![0, 0, 1]];

        assert_eq!(find_min_solution(solutions), 3);
    }
}
