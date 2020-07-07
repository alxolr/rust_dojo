fn som(x: i64, y: i64) -> i64 {
    x + y
}

fn maxi(x: i64, y: i64) -> i64 {
    if x > y {
        x
    } else {
        y
    }
}

fn mini(x: i64, y: i64) -> i64 {
    if x < y {
        x
    } else {
        y
    }
}
fn gcd(mut x: i64, mut y: i64) -> i64 {
    while x != y {
        if x > y {
            x = x - y;
        } else {
            y = y - x;
        }
    }

    x
}

fn gcdi(m: i64, n: i64) -> i64 {
    gcd(m.abs(), n.abs())
}

fn lcm(a: i64, b: i64) -> i64 {
    if a == 0 || b == 0 {
        0
    } else {
        (a * b) / gcd(a, b)
    }
}

fn lcmu(a: i64, b: i64) -> i64 {
    lcm(a.abs(), b.abs())
}

// first parameter: dots have to be replaced by function of two variables
pub fn oper_array<F: Fn(i64, i64) -> i64>(f: F, a: &[i64], init: i64) -> Vec<i64> {
    let mut v: Vec<i64> = Vec::new();

    for (idx, x) in a.iter().enumerate() {
        if idx == 0 {
            v.push(f(*x, init));
        } else {
            v.push(f(*x, v[idx - 1]));
        }
    }

    v
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing_som(a: &[i64], exp: &Vec<i64>) -> () {
        assert_eq!(&oper_array(som, a, 0), exp);
    }
    fn testing_lcmu(a: &[i64], exp: &Vec<i64>) -> () {
        assert_eq!(&oper_array(lcmu, a, a[0]), exp);
    }
    fn testing_gcdi(a: &[i64], exp: &Vec<i64>) -> () {
        assert_eq!(&oper_array(gcdi, a, a[0]), exp);
    }
    fn testing_maxi(a: &[i64], exp: &Vec<i64>) -> () {
        assert_eq!(&oper_array(maxi, a, a[0]), exp);
    }

    #[test]
    fn basics_som() {
        testing_som(&[18, 69, -90, -78, 65, 40], &vec![18, 87, -3, -81, -16, 24]);
    }
    #[test]
    fn basics_lcmu() {
        testing_lcmu(
            &[18, 69, -90, -78, 65, 40],
            &vec![18, 414, 2070, 26910, 26910, 107640],
        );
    }
    #[test]
    fn basics_maxi() {
        testing_maxi(&[18, 69, -90, -78, 65, 40], &vec![18, 69, 69, 69, 69, 69]);
    }
    #[test]
    fn basics_gcdi() {
        testing_gcdi(&[18, 69, -90, -78, 65, 40], &vec![18, 3, 3, 3, 1, 1]);
    }
}
