pub fn solve_day_1(lines: impl Iterator<Item = Result<String, std::io::Error>>) -> i32 {
    let x = lines.flatten().flat_map(|l| l.parse::<i32>()).fold(
        (None::<i32>, None::<i32>, None::<i32>, 0),
        |a, c| match a {
            (Some(a0), Some(a1), Some(a2), a3) => {
                let s1 = a0 + a1 + a2;
                let s2 = c + a0 + a1;

                let count = if s2 > s1 { a.3 + 1 } else { a3 };

                (Some(c), Some(a0), Some(a1), count)
            }
            (Some(a0), Some(a1), None, a3) => (Some(c), Some(a0), Some(a1), a3),
            (Some(a0), None, None, a3) => (Some(c), Some(a0), None, a3),
            (_, _, _, a3) => (Some(c), None, None, a3),
        },
    );

    x.3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_data() {
        let lines = vec![
            Ok("199".to_string()),
            Ok("200".to_string()),
            Ok("208".to_string()),
            Ok("210".to_string()),
            Ok("200".to_string()),
            Ok("207".to_string()),
            Ok("240".to_string()),
            Ok("269".to_string()),
            Ok("260".to_string()),
            Ok("263".to_string()),
        ]
        .into_iter();

        let expected = 5;
        let result = solve_day_1(lines);

        assert_eq!(expected, result);
    }
}
