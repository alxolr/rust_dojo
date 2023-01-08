use std::collections::HashMap;

struct Solution;

struct Point(i32, i32);

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl ToString for Point {
    fn to_string(&self) -> String {
        format!("{}{}", self.0, self.1)
    }
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        let len = points.len();
        let points = points
            .into_iter()
            .map(|item| Point(item[0], item[1]))
            .collect::<Vec<_>>();
        let mut slope: HashMap<String, i32> = HashMap::new();

        if len < 2 {
            return len as i32;
        }

        let mut max_point = 0;

        for i in 0..len {
            let mut cur_max = 0;
            let mut overlap_points = 0;
            let mut vertical_points = 0;

            for j in i + 1..len {
                if points[i] == points[j] {
                    overlap_points += 1;
                } else if points[i].0 == points[j].0 {
                    vertical_points += 1;
                } else {
                    let x_diff = points[j].0 - points[i].0;
                    let y_diff = points[j].1 - points[i].1;

                    let g = gcd(x_diff, y_diff);

                    let x_diff = x_diff / g;
                    let y_diff = y_diff / g;

                    let tmp = Point(y_diff, x_diff);
                    let key = tmp.to_string();
                    let entry = slope.entry(key).or_insert(0);
                    *entry += 1;

                    cur_max = cur_max.max(*entry);
                }
                cur_max = cur_max.max(vertical_points);
            }

            max_point = max_point.max(cur_max + overlap_points + 1);

            slope.clear();
        }

        max_point
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            (vec![vec![1, 1], vec![2, 2], vec![3, 3]], 3),
            (
                vec![
                    vec![1, 1],
                    vec![3, 2],
                    vec![5, 3],
                    vec![4, 1],
                    vec![2, 3],
                    vec![1, 4],
                ],
                4,
            ),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::max_points(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
