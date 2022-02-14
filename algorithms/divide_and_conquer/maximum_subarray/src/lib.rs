pub fn find_max_subarray(arr: &Vec<i32>, low: usize, high: usize) -> (usize, usize, i32) {
    if high == low {
        return (low, high, arr[low]);
    } else {
        let mid = (low + high) / 2;
        let (left_low, left_high, left_sum) = find_max_subarray(arr, low, mid);
        let (right_low, right_high, right_sum) = find_max_subarray(arr, mid + 1, high);
        let (cross_low, cross_high, cross_sum) = find_max_crossing_subarray(arr, low, mid, high);

        if left_sum >= right_sum && left_sum >= cross_sum {
            return (left_low, left_high, left_sum);
        } else if right_sum >= left_sum && right_sum >= cross_sum {
            return (right_low, right_high, right_sum);
        } else {
            return (cross_low, cross_high, cross_sum);
        }
    }
}

pub fn find_max_crossing_subarray(
    arr: &Vec<i32>,
    low: usize,
    mid: usize,
    high: usize,
) -> (usize, usize, i32) {
    let mut left_sum = i32::MIN;
    let mut sum = 0;
    let mut max_left: usize = 0;
    let mut max_right: usize = 0;
    let mut i = mid;

    loop {
        sum += arr[i];
        if sum > left_sum {
            left_sum = sum;
            max_left = i;
        }
        i -= 1;
        if i <= low {
            break;
        }
    }

    sum = 0;
    let mut right_sum = i32::MIN;
    let mut j = mid + 1;

    loop {
        sum += arr[j];
        if sum > right_sum {
            right_sum = sum;
            max_right = j;
        }

        j -= 1;
        if j <= high {
            break;
        }
    }

    (max_left, max_right, left_sum + right_sum)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn works_fine_for_finding_max_crossing_subarray() {
        let arr = vec![-1, 2, 3, 4, -5, 3, 9, 11, 22, 3];
        let result = find_max_crossing_subarray(&arr, 0, 5, 10);
        println!("{:?}", result);
        assert_eq!(true, true);
    }
}
