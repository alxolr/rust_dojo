pub fn max_sum_sub_array(nums: Vec<i32>) -> i32 {
    let mut sum_so_far = i32::MIN;
    let mut sum_here = 0;

    for i in 0..nums.len() {
        sum_here += nums[i];

        if sum_so_far >= sum_so_far {
            sum_so_far = sum_here;
        }

        if sum_here < 0 {
            sum_here = 0;
        }
    }

    sum_so_far
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = max_sum_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]);
        assert_eq!(result, 6);
    }

    #[test]
    fn checks_the_negative_fields() {
        let result = max_sum_sub_array(vec![-1, 0, -2]);
        assert_eq!(result, 0);
    }

    #[test]
    fn checks_the_other_scenario() {
        let result = max_sum_sub_array(vec![8, -19, 5, -4, 20]);
        assert_eq!(result, 21);
    }
}
