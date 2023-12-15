use std::time::Instant;

use solution::rod_cut;

use crate::solution::memo_hits;

mod solution;

fn main() {
    let prices = vec![1, 5, 8, 9, 10, 17, 17, 20, 24, 30];
    let n = 8;

    let now = Instant::now();
    let result = rod_cut(n, &prices);

    println!(
        "Given \nprices {:?}\nn = {}, max profit = {}\ntime {:.2?}\nmemo_hits = {}",
        prices,
        n,
        result,
        now.elapsed(),
        memo_hits().lock().unwrap(),
    );
}
