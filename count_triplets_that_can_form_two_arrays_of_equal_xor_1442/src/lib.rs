pub struct Solution;

impl Solution {
    pub fn count_triplets(arr: Vec<i32>) -> i32 {
        let len = arr.len();
        let mut res = 0;

        for i in 0..len {
            let mut cur_xor = arr[i];

            for k in i + 1..len {
                cur_xor ^= arr[k];

                if cur_xor == 0 {
                    res += (k - i) as i32
                }
            }
        }

        res
    }  
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![(vec![2, 3, 1, 6, 7], 4), (vec![1, 1, 1, 1, 1], 10)];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::count_triplets(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
