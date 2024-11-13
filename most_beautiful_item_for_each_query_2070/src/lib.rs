pub struct Solution;

// Ideas
// Transform the items into a hashmap where the price will be key and value
// will be max beauty
impl Solution {
    pub fn maximum_beauty(items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let mut items = items;

        items.sort_by(|l, r| l[0].cmp(&r[0]));

        // compute the max seen so far
        let mut max_beauty = items[0][1];
        for item in items.iter_mut() {
            max_beauty = max_beauty.max(item[1]);
            item[1] = max_beauty;
        }

        let mut ans = Vec::with_capacity(queries.len());

        for query in queries {
            ans.push(Self::binary_search(&items, query));
        }

        ans
    }

    fn binary_search(items: &Vec<Vec<i32>>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = items.len() - 1;
        let mut max_beauty = 0;

        while left <= right {
            let mid = (left + right) / 2;
            if items[mid][0] > target {
                right = mid - 1;
            } else {
                // Found viable price. Keep moving to right
                max_beauty = i32::max(max_beauty, items[mid][1]);
                left = mid + 1;
            }
        }

        max_beauty
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            (
                (
                    vec![vec![1, 2], vec![3, 2], vec![2, 4], vec![5, 6], vec![3, 5]],
                    vec![1, 2, 3, 4, 5, 6],
                ),
                vec![2, 4, 5, 5, 6, 6],
            ),
            (
                (
                    vec![vec![1, 2], vec![1, 2], vec![1, 3], vec![1, 4]],
                    vec![1],
                ),
                vec![4],
            ),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((items, queries), expected))| {
                let result = Solution::maximum_beauty(items, queries);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
