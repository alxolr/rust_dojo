pub struct Solution;

impl Solution {

    fn dp(idx: usize, books: &Vec<Vec<i32>>, shelf_width: i32) -> i32 {
        if idx == books.len() {
            return 0;
        }

        let mut result = i32::MAX;
        let curr_width = shelf_width;
        let mut max_height = 0;

        for j




        1
    }

    pub fn min_height_shelves(books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![(
            (
                vec![
                    vec![1, 1],
                    vec![2, 3],
                    vec![2, 3],
                    vec![1, 1],
                    vec![1, 1],
                    vec![1, 1],
                    vec![1, 2],
                ],
                4,
            ),
            6,
        )];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((books, shelf_width), expected))| {
                let result = Solution::min_height_shelves(books, shelf_width);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
