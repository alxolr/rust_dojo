use std::collections::BinaryHeap;

pub fn min_stone_sum(piles: Vec<i32>, k: i32) -> i32 {
    let op = |value: &i32| value - value / 2;

    let mut heap = BinaryHeap::from(piles);
    let mut op_count = k;

    while op_count.gt(&0) {
        let value = op(&heap.pop().unwrap());
        heap.push(value);
        op_count -= 1;
    }

    heap.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_stone_sum() {
        let piles = vec![5, 4, 9];
        assert_eq!(min_stone_sum(piles, 2), 12);
    }

    #[test]
    fn test_size_one() {
        let piles = vec![10000];
        assert_eq!(min_stone_sum(piles, 1), 5000);
    }

    #[test]
    fn test_other_test_case() {
        let piles = vec![4122, 9928, 3477, 9942];
        assert_eq!(min_stone_sum(piles, 6), 8768);
    }
}
