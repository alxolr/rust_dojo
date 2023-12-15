# Dynamic Programming 

## A list of easy to understand DP problems to get started with DP

### 1. Rod Cutting Problem

Given a rod of length n inches and an array of prices that contains prices of all pieces of size smaller than n. Determine the maximum value obtainable by cutting up the rod and selling the pieces.

*Example*:

 sizes[] = {1, 2, 3, 4, 5,  6,  7,  8}
 price[] = {1, 5, 8, 9, 10, 17, 17, 20}

n = 8 inches
Output: 22

*Explanation*: The maximum obtainable is by cutting in two pieces of lengths 2 and 6, i.e., 5+17=22
Solution: [Rod Cutting Problem](./rod_cut/src/solution.rs)

```rust
fn rod_cut(prices:&[i64], n:i64) -> i64 {
    if n <= 0 {
        return 0;
    }

    let profit = (1..=n).fold(-1, |acc, i| {
        acc.max(prices.get(i as usize - 1).unwrap_or(&0) + rod_cut(prices, n - i))
    });

    profit
}
```

With memoization:

```rust
pub fn rod_cut(n: i64, prices: &[i64]) -> i64 {
    let mut memo = HashMap::new();

    _rod_cut_memo(&mut memo, n, prices)
}

fn _rod_cut_memo(memo: &mut HashMap<i64, i64>, n: i64, prices: &[i64]) -> i64 {
    if let Some(result) = memo.get(&n) {
        return *result;
    }

    if n <= 0 {
        return 0;
    }

    let profit = (1..=n).fold(-1, |acc, i| {
        acc.max(prices.get(i as usize - 1).unwrap_or(&0) + _rod_cut_memo(memo, n - i, prices))
    });

    memo.insert(n, profit);

    profit
}
```



