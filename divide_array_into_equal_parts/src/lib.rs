struct Solution;

impl Solution {
    pub fn even_odd_bit(n: i32) -> Vec<i32> {
        let mut bit_counter = vec![0; 10];

        count_bits(n, &mut bit_counter);

        let (even, odd): (Vec<_>, Vec<_>) = bit_counter
            .iter()
            .enumerate()
            .partition(|(idx, _)| idx % 2 == 0);

        let even = even.iter().map(|(_, n)| *n).sum::<i32>();
        let odd = odd.iter().map(|(_, n)| *n).sum::<i32>();

        vec![even, odd]
    }
}

fn count_bits(mut num: i32, bit_counter: &mut Vec<i32>) {
    let mut bit_idx = 0;

    while num > 0 {
        let bit_value = num % 2;

        if bit_value == 1 {
            bit_counter[bit_idx] += 1;
        }
        bit_idx += 1;

        num /= 2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::even_odd_bit(2);

        assert_eq!(result, vec![0, 1]);
    }
}
