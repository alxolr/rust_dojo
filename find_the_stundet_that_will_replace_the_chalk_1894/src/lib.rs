pub struct Solution;
impl Solution {
    pub fn chalk_replacer(chalk: Vec<i32>, k: i32) -> i32 {
         // Calculate the total amount of chalk used in one full cycle
         let mut total_chalk = 0;
         for &amount in &chalk {
             total_chalk += amount;
             if total_chalk > k {
                 break;
             }
         }
 
         // Find the remaining chalk after full cycles
         let mut remaining_chalk = k % total_chalk;
 
         // Determine which student will use the remaining chalk
         for (idx, &chalk_amount) in chalk.iter().enumerate() {
             if remaining_chalk < chalk_amount {
                 return idx as i32;
             }
             remaining_chalk -= chalk_amount;
         }
 
         // This line should never be reached if the input is valid
         unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![((vec![5, 1, 5], 22), 0), ((vec![3, 4, 1, 2], 25), 1)];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((chalk, k), expected))| {
                let result = Solution::chalk_replacer(chalk, k);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
