struct Solution;

impl Solution {
    pub fn find_content_children(greeds: Vec<i32>, sizes: Vec<i32>) -> i32 {
        let mut greeds = greeds;
        let mut sizes = sizes;

        if greeds.len() == 0 {
            return 0;
        }

        if sizes.len() == 0 {
            return 0;
        }

        greeds.sort();
        sizes.sort();

        let mut content_count = 0;

        let mut i = 0; // used to iterate greeds
        let mut j = 0; // used to iterate sizes

        loop {
            if greeds[i] <= sizes[j] {
                content_count += 1;
                i += 1;
                j += 1;
            } else {
                j += 1;
            }

            if j > sizes.len() - 1 {
                break;
            }

            if i > greeds.len() - 1 {
                break;
            }
        }

        content_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_content_children_ok() {
        let scenarios = vec![
            ((vec![1, 2, 3], vec![1, 1]), 1),
            ((vec![1, 2, 3], vec![]), 0),
            ((vec![1, 2], vec![1, 2, 3]), 2),
            ((vec![10, 9, 8, 7], vec![5, 6, 7, 8]), 2),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((g, s), expected))| {
                let result = Solution::find_content_children(g, s);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
