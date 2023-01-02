struct Solution;

fn find_solutions(solutions: &mut Vec<Vec<i32>>, size: i32) {
    if size > 0 {
        let current = solutions.last().unwrap();

        let mut right = current.clone();
        let mut left = current.clone();

        left.insert(0, 0);
        right.push(0);

        let solution = left
            .iter()
            .zip(right.iter())
            .map(|(l, r)| *l + r)
            .collect::<Vec<_>>();

        solutions.push(solution);

        find_solutions(solutions, size - 1);
    }
}

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        match num_rows {
            0 => vec![],
            _ => {
                let mut solutions = vec![vec![1]];
                find_solutions(&mut solutions, num_rows - 1);

                solutions
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::generate(5);
        assert_eq!(
            result,
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1]
            ]
        );
    }
}
