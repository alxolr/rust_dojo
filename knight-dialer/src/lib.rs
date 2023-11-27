struct Solution;

const MOD: i32 = 1_000_000_007;

impl Solution {
    pub fn knight_dialer(n: i32) -> i32 {
        let jumps = vec![
            vec![4, 6],
            vec![6, 8],
            vec![7, 9],
            vec![4, 8],
            vec![0, 3, 9],
            vec![],
            vec![0, 1, 7],
            vec![2, 6],
            vec![1, 3],
            vec![2, 4],
        ];

        let mut memo = vec![vec![0; 10]; n as usize];

        let mut ans = 0;

        for i in 0..10 {
            ans += dp(&mut memo, &jumps, n - 1, i);
            ans %= MOD;
        }

        ans
    }
}

fn dp(memo: &mut Vec<Vec<i32>>, jumps: &Vec<Vec<i32>>, remain: i32, place: i32) -> i32 {
    if remain == 0 {
        return 1;
    }

    let mut ans = 0;

    for next_place in jumps[place as usize].iter() {
        if memo[remain as usize][*next_place as usize] == 0 {
            memo[remain as usize][*next_place as usize] =
                dp(memo, jumps, remain - 1, *next_place);
        }

        ans += memo[remain as usize][*next_place as usize];
        ans %= MOD;
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_knight_dialer_ok() {
        assert_eq!(Solution::knight_dialer(1), 10);
        assert_eq!(Solution::knight_dialer(2), 20);
        assert_eq!(Solution::knight_dialer(3131), 136006598);
    }
}
