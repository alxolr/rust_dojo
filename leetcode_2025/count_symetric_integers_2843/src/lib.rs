pub struct Solution;

fn is_symetric(item: &i32) -> bool {
    let digits: Vec<u32> = item.to_string().chars().filter_map(|c| c.to_digit(10)).collect();
    
    if digits.len() % 2 != 0 {
        return false;
    }
    
    let mid = digits.len() / 2;
    let left_sum: u32 = digits[..mid].iter().sum();
    let right_sum: u32 = digits[mid..].iter().sum();
    
    left_sum == right_sum
}

impl Solution {
    pub fn count_symmetric_integers(low: i32, high: i32) -> i32 {
        (low..=high).filter(is_symetric).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![((1, 100), 9), ((1200, 1230), 4)];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((low, high), expected))| {
                let result = Solution::count_symmetric_integers(low, high);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
