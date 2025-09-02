type Point = (i32, i32);
type Square = (Point, Point);

impl Solution {
    pub fn number_of_pairs(points: Vec<Vec<i32>>) -> i32 {
        let mut good_pairs = 0;

        for first_point in points.iter().map(|point| (point[0], point[1])) {
            for second_point in points.iter().map(|point| (point[0], point[1])) {
                if first_point == second_point {
                    continue;
                }

                if !Self::is_point_upper_left(&first_point, &second_point) {
                    continue;
                }

                let square = (first_point, second_point);

                let mut is_valid = true;
                for point in points
                    .iter()
                    .map(|point| (point[0], point[1]))
                    .filter(|point| *point != first_point && *point != second_point)
                {
                    if Self::is_point_in_square(&point, &square) {
                        is_valid = false;
                        break;
                    }
                }

                if is_valid {
                    good_pairs += 1
                }
            }
        }

        good_pairs
    }

    fn is_point_in_square(point: &Point, square: &Square) -> bool {
        let (x, y) = point;
        let ((top_x, top_y), (bottom_x, bottom_y)) = square;

        x <= top_x.max(bottom_x)
            && x >= top_x.min(bottom_x)
            && y <= top_y.max(bottom_y)
            && y >= bottom_y.min(top_y)
    }

    fn is_point_upper_left(point1: &Point, point2: &Point) -> bool {
        let (x1, y1) = point1;
        let (x2, y2) = point2;

        x1 <= x2 && y1 >= y2
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_is_point_upper_left() {
        let scenarios = vec![(((1,1),(2,2)), false)];
        
        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::is_point_upper_left(&input.0, &input.1);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }

    #[test]
    fn test_is_in_square() {
        let scenarios = vec![
            (((4, 4), ((2, 6), (6, 2))), true),
            (((1, 6), ((2, 6), (6, 2))), false),
            (((1, 6), ((0, 6), (6, 2))), true),
            (((6, 6), ((0, 6), (6, 2))), true),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::is_point_in_square(&input.0, &input.1);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }

    #[test]
    fn it_works() {
        let scenarios = vec![
            (vec![vec![1, 1], vec![2, 2], vec![3, 3]], 0),
            (vec![vec![6, 2], vec![4, 4], vec![2, 6]], 2),
            (vec![vec![3, 1], vec![1, 3], vec![1, 1]], 2),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::number_of_pairs(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
