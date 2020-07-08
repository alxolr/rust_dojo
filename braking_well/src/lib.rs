const G: f64 = 9.81;

fn dist(v: f64, mu: f64) -> f64 {
    kmh_to_ms(v) + kmh_to_ms(v).powi(2) / (2. * mu * G)
}

fn speed(d: f64, mu: f64) -> f64 {
    let a = 1. / (2. * mu * G);
    let b = 1.;
    let c = -d;

    // quadratic formula
    let v = (-b + (b * b - 4. * a * c).sqrt()) / (2. * a);

    ms_to_kmh(v)
}

fn kmh_to_ms(v: f64) -> f64 {
    v / 3.6
}

fn ms_to_kmh(v: f64) -> f64 {
    v * 3.6
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_fuzzy_equals(actual: f64, expected: f64) {
        let merr = 1.0e-12;
        let inrange = if expected == 0.0 {
            actual.abs() <= merr
        } else {
            (actual - expected).abs() / expected <= merr
        };
        if inrange == false {
            println!(
                "Expected value must be near: {:e} but was:{:e}",
                expected, actual
            );
        } else {
        }
        assert_eq!(true, inrange);
    }

    fn dotest1(v: f64, mu: f64, exp: f64) -> () {
        assert_fuzzy_equals(dist(v, mu), exp);
    }
    fn dotest2(d: f64, mu: f64, exp: f64) -> () {
        assert_fuzzy_equals(speed(d, mu), exp);
    }

    #[test]
    fn basic_tests_dist() {
        dotest1(144.0, 0.3, 311.83146449201496);
        dotest1(92.0, 0.5, 92.12909477605366);
    }
    #[test]
    fn basic_tests_speed() {
        dotest2(159.0, 0.8, 153.79671564846308);
    }
}
