use std::{cmp::Reverse, collections::BinaryHeap};

pub struct KthLargest {
    data: BinaryHeap<Reverse<i32>>,
    k: i32,
}

impl KthLargest {
    pub fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut data: BinaryHeap<i32> = BinaryHeap::new();

        for it in nums {
            data.push(it)
        }

        KthLargest { k, data }
    }

    pub fn add(&mut self, val: i32) -> i32 {
        self.data.push(val);

        println!("{:?}", self.data);
        let val = self.data.iter().nth(self.k as usize).unwrap();

        *val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut kth_largest = KthLargest::new(3, vec![4, 5, 8, 2]);
        assert_eq!(kth_largest.add(3), 4);
        assert_eq!(kth_largest.add(5), 5);
        assert_eq!(kth_largest.add(10), 5);
        assert_eq!(kth_largest.add(9), 8);
        assert_eq!(kth_largest.add(4), 8);
    }
}
