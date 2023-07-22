pub fn search<T>(collection: &Vec<T>, item: T) -> usize
where
    T: PartialOrd + PartialEq,
{
    let mut low = 0;
    let mut high = collection.len() - 1;

    while low <= high {
        let mid = (low + high) / 2;
        let guess = &collection[mid];

        if guess == &item {
            return mid;
        } else if guess > &item {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }

    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let collection = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

        assert_eq!(search(&collection, 1), 0);
        assert_eq!(search(&collection, 8), 7);
    }
}
