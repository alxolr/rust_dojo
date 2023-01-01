pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    if n == 0 || m == 0 {
        if m == 0 {
            for j in 0..n {
                nums1[j as usize] = nums2[j as usize];
            }
        }
    } else {
        let left = &(nums1.clone())[0..m as usize];
        let right = &(nums2.clone())[0..n as usize];

        let mut i = 0;
        let mut j = 0;

        for n in 0..nums1.len() {
            if i < left.len() && j < right.len() {
                if left[i] > right[j] {
                    nums1[n] = right[j];
                    j += 1;
                } else {
                    nums1[n] = left[i];
                    i += 1;
                }
            } else if j >= right.len() {
                nums1[n] = left[i];
                i += 1;
            } else {
                nums1[n] = right[j];
                j += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let m = 3;
        let mut nums2 = vec![2, 5, 6];
        let n = 3;

        merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
    }

    #[test]
    fn test_case_two() {
        let mut nums1 = vec![1];
        let m = 1;
        let mut nums2 = vec![];
        let n = 0;

        merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![1]);
    }

    #[test]
    fn test_case_three() {
        let mut nums1 = vec![0];
        let m = 0;
        let mut nums2 = vec![1];
        let n = 1;

        merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![1]);
    }

    #[test]
    fn test_case_four() {
        let mut nums1 = vec![4, 0, 0, 0, 0, 0];
        let m = 1;
        let mut nums2 = vec![1, 2, 3, 5, 6];
        let n = 5;

        merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![1, 2, 3, 4, 5, 6]);
    }
}
