pub struct Solution;

impl Solution {
    fn find_min_max(arr: &[i32]) -> (i32, i32) {
        (*arr.first().unwrap(), *arr.last().unwrap())
    }

    pub fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
        let mut max_distance = i32::MIN;

        let (mut global_min, mut global_max) = Self::find_min_max(&arrays[0]);

        for arr in arrays.into_iter().skip(1) {
            let (local_min, local_max) = Self::find_min_max(&arr);
            let distance = i32::max((global_min - local_max).abs(), (local_min - global_max).abs());

            global_min = global_min.min(local_min);
            global_max = global_max.max(local_max);

            max_distance = max_distance.max(distance);
        }

        max_distance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            (vec![vec![1, 2, 3], vec![4, 5], vec![1, 2, 3]], 4),
            (vec![vec![1], vec![1]], 0),
            (vec![vec![1, 4], vec![0, 5]], 4),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::max_distance(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
