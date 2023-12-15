use std::collections::HashMap;

use std::sync::{Mutex, OnceLock};

/// This construction it's a form of singleton pattern used in rust,
/// the goal is to count how many times the memoization was used.
/// It's purely a debug tool for academic purposes.
pub fn memo_hits() -> &'static Mutex<usize> {
    static MEMO_HITS: OnceLock<Mutex<usize>> = OnceLock::new();

    MEMO_HITS.get_or_init(|| Mutex::new(0))
}

pub fn rod_cut(n: i64, prices: &Vec<i64>) -> i64 {
    let mut memo = HashMap::new();

    _rod_cut(&mut memo, n, prices)
}

fn _rod_cut(memo: &mut HashMap<i64, i64>, n: i64, prices: &Vec<i64>) -> i64 {
    if let Some(result) = memo.get(&n) {
        *memo_hits().lock().unwrap() += 1;

        return *result;
    }

    if n <= 0 {
        return 0;
    }

    let profit = (1..=n).fold(-1, |acc, i| {
        acc.max(prices.get(i as usize - 1).unwrap_or(&0) + _rod_cut(memo, n - i, prices))
    });

    memo.insert(n, profit);

    profit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_rod_cut_ok() {
        let prices = vec![1, 5, 8, 9, 10, 17, 17, 20, 24, 30];
        let n = 8;
        let result = rod_cut(n, prices);

        assert_eq!(result, 22);
    }
}
