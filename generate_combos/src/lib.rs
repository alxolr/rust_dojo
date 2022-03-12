pub fn get_all_permutations<T: Clone + PartialEq>(bunch: &[T]) -> Vec<Vec<T>> {
    let mut solutions = vec![];

    if bunch.len() == 1 {
        solutions.push(vec![bunch[0].clone()])
    }

    for item in bunch {
        let mut owned_bunch = bunch.to_owned();
        let vectored_item = vec![item.clone()];

        let position = owned_bunch.iter().position(|x| x == item).unwrap();
        owned_bunch.remove(position);

        get_all_permutations(&owned_bunch)
            .into_iter()
            .for_each(|x| {
                let mut vec = vectored_item.clone();
                vec.extend(x);

                solutions.push(vec);
            });
    }

    solutions
}

#[cfg(test)]
mod tests {
    use crate::get_all_permutations;

    #[test]
    fn it_works() {
        let result = get_all_permutations(&vec![1, 0, 1, 2, 9]);

        println!("{:?}", result);

        assert_eq!(true, true);
    }
}
