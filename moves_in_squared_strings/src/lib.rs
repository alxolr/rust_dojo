fn oper<F>(f: F, s: &str) -> String
where
    F: Fn(&str) -> String,
{
    f(s)
}

fn selfie_and_diag1(s: &str) -> String {
    let diag_1 = diag_1_sym(s);
    let d = as_matrix(&diag_1);
    let l = as_matrix(s);

    let t = l
        .into_iter()
        .zip(d.into_iter())
        .map(|(x, y)| {
            let f: String = x.iter().collect();
            let l: String = y.iter().collect();

            format!("{}|{}", f, l)
        })
        .collect::<Vec<String>>()
        .join("\n");

    t
}

fn diag_1_sym(s: &str) -> String {
    let l = as_matrix(s);
    let mut t: Vec<Vec<char>> = l.clone();
    let n = l.len();

    for i in 0..n {
        for j in 0..n {
            t[i][j] = l[j][i]
        }
    }

    matrix_to_string(t)
}

fn rot_90_clock(s: &str) -> String {
    let l = as_matrix(s);
    let mut t: Vec<Vec<char>> = l.clone();
    let n = l.len();

    for i in 0..n {
        for j in 0..n {
            t[i][j] = l[n - j - 1][i]
        }
    }

    matrix_to_string(t)
}

fn as_matrix(s: &str) -> Vec<Vec<char>> {
    s.split("\n")
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn matrix_to_string(m: Vec<Vec<char>>) -> String {
    m.iter()
        .map(|l| l.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testing_rot_90_clock() {
        let l = "abcd\nefgh\nijkl\nmnop";
        let e = "miea\nnjfb\nokgc\nplhd".to_string();
        assert_eq!(rot_90_clock(l), e);
    }

    #[test]
    fn testing_diag_1_sym() {
        let l = "abcd\nefgh\nijkl\nmnop";
        let e = "aeim\nbfjn\ncgko\ndhlp".to_string();
        assert_eq!(diag_1_sym(l), e);
    }

    #[test]
    fn testing_selfie_and_diag_1_sym() {
        let l = "abcd\nefgh\nijkl\nmnop";
        let e = "abcd|aeim\nefgh|bfjn\nijkl|cgko\nmnop|dhlp".to_string();
        assert_eq!(selfie_and_diag1(l), e);
    }
}
