pub struct MinStack {
    data: Vec<i32>,
    // will store the min value for each stack level from data
    min_data: Vec<i32>,
}

impl Default for MinStack {
    fn default() -> Self {
        Self::new()
    }
}

impl MinStack {
    pub fn new() -> Self {
        MinStack {
            data: vec![],
            min_data: vec![],
        }
    }

    pub fn push(&mut self, val: i32) {
        if self.data.is_empty() {
            self.data.push(val);
            self.min_data.push(val);
        } else {
            self.data.push(val);
            if let Some(top) = self.min_data.last() {
                self.min_data.push(val.min(*top));
            }
        }
    }

    pub fn pop(&mut self) {
        if !self.data.is_empty() {
            self.data.pop();
            self.min_data.pop();
        }
    }

    pub fn top(&self) -> i32 {
        *self.data.last().unwrap()
    }

    pub fn get_min(&self) -> i32 {
        *self.min_data.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_stack_ok() {
        let mut min_stack = MinStack::new();
        min_stack.push(-2);
        min_stack.push(0);
        min_stack.push(-3);

        let result = min_stack.get_min();
        assert_eq!(result, -3);

        min_stack.pop();
        assert_eq!(min_stack.top(), 0);
        assert_eq!(min_stack.get_min(), -2);
    }

    #[test]
    fn test_min_stack_new_exists() {
        MinStack::new();
    }

    #[test]
    fn test_min_stack_push_exists() {
        let mut min_stack = MinStack::new();

        min_stack.push(1);
    }

    #[test]
    fn test_min_stack_pop_ok() {
        let mut min_stack = MinStack::new();
        min_stack.push(1);
        min_stack.pop();
    }

    #[test]
    fn test_min_stack_top_ok() {
        let mut min_stack = MinStack::new();
        min_stack.push(1);
        let result = min_stack.top();

        assert_eq!(result, 1);
    }

    #[test]
    fn test_min_stack_get_min_ok() {
        let mut min_stack = MinStack::new();
        min_stack.push(1);
        min_stack.push(-3);

        let result = min_stack.get_min();
        assert_eq!(result, -3);
    }
}
