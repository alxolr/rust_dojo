impl Solution {
    pub fn max_sum_div_three(nums: Vec<i32>) -> i32 {
        let mut dp = vec![vec![0; 3]; nums.len() + 1];
        for i in 1..=nums.len() {
            let num = nums[i - 1];
            for j in 0..3 {
                dp[i][j] = dp[i - 1][j].max(dp[i - 1][(j + num as usize % 3) % 3] + num);
            }
        }
        dp[nums.len()][0]
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            (vec![4], 0),
            (vec![3, 6, 5, 1, 8], 18),
            (vec![1, 2, 3, 4, 4], 12),
            (vec![4, 1, 5, 3, 1], 12),
            (
                vec![
                    366, 809, 6, 792, 822, 181, 210, 588, 344, 618, 341, 410, 121, 864, 191, 749,
                    637, 169, 123, 472, 358, 908, 235, 914, 322, 946, 738, 754, 908, 272, 267, 326,
                    587, 267, 803, 281, 586, 707, 94, 627, 724, 469, 568, 57, 103, 984, 787, 552,
                    14, 545, 866, 494, 263, 157, 479, 823, 835, 100, 495, 773, 729, 921, 348, 871,
                    91, 386, 183, 979, 716, 806, 639, 290, 612, 322, 289, 910, 484, 300, 195, 546,
                    499, 213, 8, 623, 490, 473, 603, 721, 793, 418, 551, 331, 598, 670, 960, 483,
                    154, 317, 834, 352,
                ],
                50487,
            ),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                println!("scenario {}", idx + 1);
                assert_eq!(Solution::max_sum_div_three(input), expected);
            });
    }
}
