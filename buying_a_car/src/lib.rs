pub fn nb_months(old: i32, new: i32, saving: i32, perc: f64) -> (i32, i32) {
    if old >= new {
        return (0, old - new);
    }

    let mut old_car_depr = old as f64;
    let mut new_car_depr = new as f64;
    let mut depr: f64 = perc;
    let mut months = 1;

    loop {
        old_car_depr *= 1. - depr / 100.;
        new_car_depr *= 1. - depr / 100.;

        if new_car_depr < (old_car_depr + (months * saving) as f64) {
            break;
        }

        months += 1;

        if months % 2 == 0 {
            depr += 0.5;
        }
    }

    let remaining =
        ((old_car_depr + (months * saving) as f64) as f64 - new_car_depr).round() as i32;

    (months, remaining)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing(old: i32, new: i32, saving: i32, perc: f64, exp: (i32, i32)) -> () {
        assert_eq!(nb_months(old, new, saving, perc), exp)
    }

    #[test]
    fn basics_nb_months() {
        testing(2000, 8000, 1000, 1.5, (6, 766));
        testing(12000, 8000, 1000, 1.5, (0, 4000));
        testing(8000, 12000, 500, 1.0, (8, 597));
        testing(18000, 32000, 1500, 1.25, (8, 332));
        testing(7500, 32000, 300, 1.55, (25, 122));
    }
}
