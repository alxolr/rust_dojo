#[derive(Debug, PartialEq)]
pub struct Interval(pub u64, pub u64);

impl Interval {
    fn intersect(&self, rhs: &Interval) -> Option<Interval> {
        let Interval(ss, se) = &self;
        let Interval(os, oe) = rhs;

        if os > se || ss > oe {
            None
        } else {
            let start = ss.max(os);
            let end = se.min(oe);

            Some(Interval(*start, *end))
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersect_two_intervals_ok() {
        let interval_1 = Interval(3, 8);
        let interval_2 = Interval(2, 6);

        assert_eq!(Interval(3, 6), interval_2.intersect(&interval_1).unwrap());
    }
}
