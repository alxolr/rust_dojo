pub fn palindrom(s: &str) -> bool {
    let other = s.chars().rev().collect::<String>();

    &other == s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = palindrom("aa");
        assert_eq!(result, true);
    }

    #[test]
    fn it_works_aba() {
        let result = palindrom("aba");
        assert_eq!(result, true);
    }

    #[test]
    fn it_works_abra() {
        let result = palindrom("abra");
        assert_eq!(result, false);
    }
}
