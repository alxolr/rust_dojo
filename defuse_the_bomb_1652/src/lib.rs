// Difuse the bomb 1652

pub struct Solution;
impl Solution {
    pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
        // We are talking about an algorithm of O(n*k) which in our case n 100 * 100
        // the constraints are pretty low so we can just try to implement it in this way.
        let mut ans: Vec<i32> = Vec::with_capacity(code.len());
        
        match k {
            k if k > 0 => {
                let circular_code = code.iter().cycle();
                for idx in 1..=code.len() {
                    let val = circular_code
                        .clone()
                        .skip(idx)
                        .take(k as usize)
                        .sum::<i32>();
                    ans.push(val);
                }

                ans
            }
            k if k == 0 => {
                vec![0; code.len()]
            }
            _ => {
                let k = (-1 * k) as usize;
                let mut code = code;
                code.reverse();

                let circular_code = code.iter().cycle();

                for idx in 1..=code.len() {
                    let val = circular_code
                        .clone()
                        .skip(idx)
                        .take(k as usize)
                        .sum::<i32>();
                    ans.push(val);
                }

                ans.reverse();

                ans
            }
        }
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            ((vec![5, 7, 1, 4], 3), vec![12, 10, 16, 13]),
            ((vec![1, 2, 3, 4], 0), vec![0, 0, 0, 0]),
            ((vec![2, 4, 9, 3], -2), vec![12, 5, 6, 13]),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::decrypt(input.0, input.1);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
