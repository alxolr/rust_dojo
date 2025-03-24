pub struct Solution;

impl Solution {
    pub fn count_days(days: i32, meetings: Vec<Vec<i32>>) -> i32 {
        let mut meetings = meetings;

        // Sort the meetings by the start time
        meetings.sort_by(|a, b| a[0].cmp(&b[0]));

        // Merge the overlapping meetings
        let mut merged_meetings = vec![meetings[0].clone()];

        for meeting in meetings.iter().skip(1) {
            let last_merged_meeting = merged_meetings.last_mut().unwrap();

            if meeting[0] <= last_merged_meeting[1] {
                last_merged_meeting[1] = last_merged_meeting[1].max(meeting[1]);
            } else {
                merged_meetings.push(meeting.clone());
            }
        }

        let mut days_result = 0;
        let mut start_day = 1;

        for meeting in merged_meetings.iter() {
            let (start, end) = (meeting[0], meeting[1]);

            if start_day < start {
                days_result += start - start_day;
            }

            start_day = end + 1;
        }

        if start_day <= days {
            days_result += days - start_day + 1;
        }

        days_result
        // Count the days
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![((10, vec![vec![5, 7], vec![1, 3], vec![9, 10]]), 2)];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((days, meetings), expected))| {
                let result = Solution::count_days(days, meetings);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
