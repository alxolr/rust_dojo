impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let n = chars.len();
        let mut prefix_sum = vec![-1; n + 1];
        for i in 0..n {
            if i == 0 || chars[i - 1] == '0' {
                prefix_sum[i + 1] = i as i32;
            } else {
                prefix_sum[i + 1] = prefix_sum[i];
            }
        }

        let mut res = 0i32;
        for i in 1..=n {
            let mut cnt0 = if chars[i - 1] == '0' { 1 } else { 0 };
            let mut j = i as i32;
            while j > 0 && (cnt0 * cnt0) as usize <= n {
                let cnt1 = (i as i32 - prefix_sum[j as usize]) - cnt0;
                if cnt0 * cnt0 <= cnt1 {
                    res += std::cmp::min(j - prefix_sum[j as usize], cnt1 - cnt0 * cnt0 + 1);
                }
                j = prefix_sum[j as usize];
                cnt0 += 1;
            }
        }
        res
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![("00011", 5), ("101101", 16)];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::number_of_substrings(input.to_string());
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
