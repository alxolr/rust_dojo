struct Solution;

impl Solution {
    pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        let n = mat.len();
        let m = mat[0].len();

        if n * m != (r * c) as usize {
            mat
        } else {
            let mut matrix = vec![vec![0; c as usize]; r as usize];
            let mut lr = 0;
            let mut lc = 0;

            for i in 0..n {
                for j in 0..m {
                    matrix[lr][lc] = mat[i][j];

                    if lc < c as usize - 1 {
                        lc += 1;
                    } else {
                        lr += 1;
                        lc = 0;
                    }
                }
            }

            matrix
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            ((vec![vec![1, 2], vec![3, 4]], 1, 4), vec![vec![1, 2, 3, 4]]),
            (
                (vec![vec![1, 2], vec![3, 4]], 2, 4),
                vec![vec![1, 2], vec![3, 4]],
            ),
        ];

        scenarios.into_iter().for_each(|(input, expected)| {
            let (mat, r, c) = input;

            assert_eq!(Solution::matrix_reshape(mat, r, c), expected);
        })
    }
}
