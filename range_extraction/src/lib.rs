

mod solution {
    pub fn range_extraction(vec: &[i32]) -> String {
        let mut stack = vec![];
        let mut result: Vec<String> = vec![];

        for value in vec {
            match stack.len() {
                0 => stack.push(value),
                _ => {
                    let last = *stack.last().unwrap();
                    if *last == (value - 1) {
                        stack.push(value);
                    } else {
                        drain_stack(stack, &mut result);
                        stack = vec![value];
                    }
                }
            }
        }

        drain_stack(stack, &mut result);

        result.join(",")
    }

    fn drain_stack(stack: Vec<&i32>, result: &mut Vec<String>) {
        if stack.len() <= 2 {
            for item in stack {
                result.push(item.to_string());
            }
        } else {
            let first = stack.first().unwrap();
            let last = stack.last().unwrap();

            result.push(format!("{}-{}", first, last));
        }
    }
}
