use rand::distributions::Distribution;
use rand::distributions::Uniform;
use std::collections::HashMap;
use std::iter;

struct RandomizedSet {
    pub data: HashMap<i32, bool>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        if self.data.contains_key(&val) {
            false
        } else {
            self.data.insert(val, true);

            true
        }
    }

    fn remove(&mut self, val: i32) -> bool {
        if self.data.contains_key(&val) {
            self.data.remove(&val);

            true
        } else {
            false
        }
    }

    fn get_random(&self) -> i32 {
        let len = self.data.len();
        let step = Uniform::new(0, len);
        let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
        let idx = step.sample(&mut rng);

        let result = self
            .data
            .keys()
            .skip(idx)
            .take(1)
            .map(|x| *x)
            .collect::<Vec<_>>();

        result[0]
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_scan_operator() {
        let item = (1..10)
            .scan(20, |state, item| {
                *state += item;

                Some(*state)
            })
            .collect::<Vec<_>>();

        println!("{:?}", item);
    }
}
