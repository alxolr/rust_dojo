pub fn is_valid_expression(s: &str) -> bool {
    let mut stack: Vec<char> = vec![];

    for current in s.chars() {
        if stack.is_empty() {
            stack.push(current);
        } else {
            let last = stack.pop().unwrap();
            if !is_complement(last, current) {
                stack.push(last);
                stack.push(current);
            }
        }
    }

    if stack.is_empty() {
        true
    } else {
        false
    }
}

fn is_complement(right: char, left: char) -> bool {
    (right == '(' && left == ')') || (right == '[' && left == ']') || (right == '{' && left == '}')
}

#[cfg(test)]
mod tests {
    use super::is_valid_expression;
    #[test]
    fn it_finds_properly_simple_brackets() {
        assert_eq!(is_valid_expression("()"), true);
        assert_eq!(is_valid_expression(")("), false);
    }

    #[test]
    fn processes_square_brackets() {
        assert_eq!(is_valid_expression("[]"), true);
        assert_eq!(is_valid_expression("}["), false);
    }

    #[test]
    fn processes_complex_brackets() {
        assert_eq!(is_valid_expression("([{}])"), true);
        assert_eq!(is_valid_expression("{{}}()[]"), true);
    }

    #[test]
    fn test_for_empty_string() {
        assert_eq!(is_valid_expression(""), true);
    }
}
