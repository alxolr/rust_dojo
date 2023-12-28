use std::collections::HashMap;

pub struct Solution;

fn climb(memo: &mut HashMap<i32, i32>, n: i32) -> i32 {
    if let Some(result) = memo.get(&n) {
        return *result;
    }

    let result = match n {
        1 => 1,
        2 => 2,
        _ => climb(memo, n - 1) + climb(memo, n - 2),
    };

    memo.insert(n, result);

    result
}

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut memo = HashMap::new();

        climb(&mut memo, n)
    }
}
