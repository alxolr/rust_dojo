pub struct Solution;

impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let mut hungry_students = students.len();
        let mut student_type_counter = students.iter().fold(vec![0; 2], |mut acc, it| {
            acc[*it as usize] += 1;

            acc
        });

        for sandwitch in sandwiches {
            if student_type_counter[sandwitch as usize] > 0 {
                hungry_students -= 1;
                student_type_counter[sandwitch as usize] -= 1;
            } else {
                return hungry_students as i32;
            }
        }

        hungry_students as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            ((vec![1, 1, 0, 0], vec![0, 1, 0, 1]), 0),
            ((vec![1, 1, 1, 1], vec![0, 1, 1, 1]), 4),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((students, sandwitches), expected))| {
                let result = Solution::count_students(students, sandwitches);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
