struct Solution;

impl Solution {
    pub fn max_coins(piles: Vec<i32>) -> i32 {
        let mut piles = piles;
        piles.sort();
        let n = piles.len();

        let mut right = piles.len() - 1;
        let mut left = 0;
        let mut result = 0;

        let mut iter = 0;
        
        while iter < (n / 3) {
            result += piles[right - 1];
            left += 1;
            right -= 2;
            iter += 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_coins_ok() {
        let piles = vec![2, 4, 1, 2, 7, 8];
        let result = Solution::max_coins(piles);
        assert_eq!(9, result);

        let piles = vec![2, 4, 5];
        let result = Solution::max_coins(piles);
        assert_eq!(4, result);

        let piles = vec![9, 8, 7, 6, 5, 1, 2, 3, 4];
        let result = Solution::max_coins(piles);
        assert_eq!(18, result);
    }
}
