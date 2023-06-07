struct Solution;

fn to_binary(n: i32) -> Vec<i32> {
    let s = format!("{:b}", n);

    s.chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect::<Vec<_>>()
}

fn prepend_zeros(v: &mut Vec<i32>, n: usize) {
    for _ in 0..n {
        v.insert(0, 0);
    }
}

impl Solution {
    pub fn min_flips(a: i32, b: i32, c: i32) -> i32 {
        let mut a = to_binary(a);
        let a_len = a.len();
        let mut b = to_binary(b);
        let b_len = b.len();
        let mut c = to_binary(c);
        let c_len = c.len();

        // make them equal length
        let max = a.len().max(b.len()).max(c.len());

        prepend_zeros(&mut a, max - a_len);
        prepend_zeros(&mut b, max - b_len);
        prepend_zeros(&mut c, max - c_len);


        println!("{:?}", a);
        println!("{:?}", b);
        println!("{:?}", c);

        let mut count = 0;


        for i in 0..max {
            if c[i] == 0 {
                if a[i] == 1 {
                    count += 1;
                }

                if b[i] == 1 {
                    count += 1;
                }
            } else {
                if a[i] == 0 && b[i] == 0 {
                    count += 1;
                }
            }
        }

        count
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(super::Solution::min_flips(2, 6, 5), 3);
        assert_eq!(super::Solution::min_flips(4, 2, 7), 1);
        assert_eq!(super::Solution::min_flips(1, 2, 3), 0);
    }
}
