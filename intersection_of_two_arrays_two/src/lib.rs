pub fn intersect(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
    let mut right = right.clone();
    let mut result = vec![];

    for i in 0..left.len() {
        if let Some((idx, value)) = right.iter().enumerate().find(|(_, item)| *item == &left[i]) {
            result.push(*value);
            right.remove(idx);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_one() {
        let result = intersect(vec![1, 2, 2, 1], vec![2, 2]);
        assert_eq!(result, vec![2, 2]);
    }

    #[test]
    fn test_case_two() {
        let result = intersect(vec![4, 9, 5], vec![9, 4, 9, 8, 4]);
        assert_eq!(result, vec![4, 9]);
    }
}
