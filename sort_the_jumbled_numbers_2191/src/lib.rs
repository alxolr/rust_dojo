pub struct Solution;

impl Solution {
    fn collect(num: &[i32]) -> i32 {
        let str = num.iter().map(|x| x.to_string()).collect::<Vec<_>>();

        let num = str.join("");

        num.parse().unwrap()
    }

    fn expand(num: i32) -> Vec<i32> {
        num.to_string()
            .chars()
            .flat_map(|c| c.to_digit(10))
            .map(|x| x as i32)
            .collect()
    }

    pub fn sort_jumbled(mapping: Vec<i32>, nums: Vec<i32>) -> Vec<i32> {
        let mut mapped_nums = nums
            .into_iter()
            .map(|num| {
                let mapped_num = Solution::expand(num)
                    .into_iter()
                    .map(|x| mapping[x as usize])
                    .collect::<Vec<_>>();

                let mapped_num = Solution::collect(&mapped_num);

                (num, mapped_num)
            })
            .collect::<Vec<_>>();

        mapped_nums.sort_by(|a, b| a.1.cmp(&b.1));

        mapped_nums.into_iter().map(|(num, _)| num).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            (
                (vec![8, 9, 4, 0, 2, 1, 3, 5, 7, 6], vec![991, 338, 38]),
                vec![338, 38, 991],
            ),
            (
                (vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], vec![789, 456, 123]),
                vec![123, 456, 789],
            ),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((mapping, nums), expected))| {
                let result = Solution::sort_jumbled(mapping, nums);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
