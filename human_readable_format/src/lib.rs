fn pluralize(v: u64, s: &str) -> String {
    if v > 1 {
        format!("{} {}s", v, s)
    } else {
        format!("{} {}", v, s)
    }
}

fn format_duration(seconds: u64) -> String {
    let mut seconds = seconds;
    let seconds_in_year = 365 * 24 * 3600;
    let years = seconds / seconds_in_year;
    seconds -= years * seconds_in_year;

    let days = seconds / (3600 * 24);
    seconds -= days * 3600 * 24;

    let hours = seconds / 3600;
    seconds -= hours * 3600;

    let minutes = seconds / 60;
    seconds -= minutes * 60;

    let values = vec![
        pluralize(years, "year"),
        pluralize(days, "day"),
        pluralize(hours, "hour"),
        pluralize(minutes, "minute"),
        pluralize(seconds, "second"),
    ];

    let filtered = values
        .into_iter()
        .filter(|v| !v.starts_with("0"))
        .collect::<Vec<_>>();

    let things_size = filtered.len();

    let last = if things_size == 0 {
        "now"
    } else {
        filtered.last().unwrap()
    };
        
    if things_size > 1 {
        let comma_separated = filtered
            .iter()
            .take(things_size - 1)
            .map(|x| x.clone())
            .collect::<Vec<_>>();

        format!("{} and {}", comma_separated.join(", "), last)
    } else {
        format!("{}", last)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(format_duration(0), "");
        assert_eq!(format_duration(1), "1 second");
        assert_eq!(format_duration(62), "1 minute and 2 seconds");
        assert_eq!(format_duration(120), "2 minutes");
        assert_eq!(format_duration(3600), "1 hour");
        assert_eq!(format_duration(3662), "1 hour, 1 minute and 2 seconds");
    }
}
