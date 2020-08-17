fn stati(strg: &str) -> String {
    if strg == "" {
        return "".to_string();
    };

    let mut time = vec![];
    strg.split(", ").for_each(|x| {
        let t = x.split("|").collect::<Vec<_>>();
        let o = t
            .iter()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        time.push(o[0] * 3600 + o[1] * 60 + o[2]);
    });

    let len = time.len();
    let middle = time.len() / 2;

    time.sort();

    let rng = to_time(time.last().unwrap() - time.first().unwrap());
    let median = if len % 2 == 0 {
        to_time((time[middle] + time[middle - 1]) / 2)
    } else {
        to_time(time[middle])
    };
    let avg = to_time(time.iter().sum::<i32>() / len as i32);
    format!("Range: {} Average: {} Median: {}", rng, avg, median)
}

fn to_time(secs: i32) -> String {
    let h = secs / 3600;
    let m = (secs - (h * 3600)) / 60;
    let s = secs - (h * 3600 + m * 60);

    format!("{:02}|{:02}|{:02}", h, m, s)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(strg: &str, exp: &str) -> () {
        println!(" str: {:?};", strg);
        let ans = stati(strg);
        println!(" actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!(" {};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn basic_tests() {
        dotest(
            "01|15|59, 1|47|16, 01|17|20, 1|32|34, 2|17|17",
            "Range: 01|01|18 Average: 01|38|05 Median: 01|32|34",
        );
        dotest(
            "02|15|59, 2|47|16, 02|17|20, 2|32|34, 2|17|17, 2|22|00, 2|31|41",
            "Range: 00|31|17 Average: 02|26|18 Median: 02|22|00",
        );
        dotest(
            "01|15|59, 1|47|16, 01|17|20, 1|32|34, 2|17|17",
            "Range: 01|01|18 Average: 01|38|05 Median: 01|32|34",
        );
        dotest(
            "02|15|59, 2|47|16, 02|17|20, 2|32|34, 2|32|34, 2|17|17",
            "Range: 00|31|17 Average: 02|27|10 Median: 02|24|57",
        );
    }
}
