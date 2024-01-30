pub fn add(left: usize, right: usize) -> usize {
    println!("{:08b} {:08b}", left, right);

    println!("{:08b} & {:08b} = {:08b}", left, right, left & right);
    println!("{:08b} | {:08b} = {:08b}", left, right, left | right);
    println!("{:08b} ^ {:08b} = {:08b}", left, right, left ^ right);
    println!("!{:08b} = {:08b}", left, !left);

    println!("{:08b} << 2 = {:08b}", left, left << 2);
    println!("{:08b} >> 2 = {:08b}", left, left >> 2);

    let arr = vec![2, 3, 2, 6, 3];
    
    let result = arr.iter().fold(0, |mut acc, item| {
        acc ^= item;

        acc
    });

    println!("{}", result);

    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(13, 16);
        assert_eq!(result, 29);
    }
}
