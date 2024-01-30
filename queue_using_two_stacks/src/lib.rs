pub struct MyQueue {
    s1: Vec<i32>,
    s2: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
    fn new() -> Self {
        Self {
            s1: vec![],
            s2: vec![],
        }
    }

    fn push(&mut self, x: i32) {
        self.s1.push(x);
    }

    fn pop(&mut self) -> i32 {
        if !self.s2.is_empty() {
            return self.s2.pop().unwrap();
        }

        while let Some(val) = self.s1.pop() {
            self.s2.push(val);
        }

        self.s2.pop().unwrap()
    }

    fn peek(&self) -> i32 {
        if !self.s2.is_empty() {
            return *self.s2.last().unwrap();
        } else {
            return *self.s1.first().unwrap();
        }
    }

    fn empty(&self) -> bool {
        self.s1.is_empty() && self.s2.is_empty()
    }
}

/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_queue_ok() {
        let mut queue = MyQueue::new();

        queue.push(4);
        let peek = queue.peek();

        assert_eq!(peek, 4)
    }

    #[test]
    fn test_my_queue_works_ok() {
        let mut queue = MyQueue::new();

        queue.push(1);
        queue.push(2);
        queue.push(3);
        queue.push(4);
        let mut vec = vec![];
        let val = queue.pop();
        vec.push(val);
        queue.push(5);

        vec.push(queue.pop());
        vec.push(queue.pop());
        vec.push(queue.pop());
        vec.push(queue.pop());
    }
}
