use std::error::Error;

pub fn solve_day_1(
    lines: impl Iterator<Item = Result<String, std::io::Error>>,
) -> Result<i32, Box<dyn Error>> {
    let count = lines
        .flatten()
        .flat_map(|l| l.parse::<i32>())
        .fold((0, None::<i32>), |a, c| {
            if let Some(p) = a.1 {
                if c > p {
                    (a.0 + 1, Some(c))
                } else {
                    (a.0, Some(c))
                }
            } else {
                (a.0, Some(c))
            }
        })
        .0;

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
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

        let expected = 7;
        let result = solve_day_1(lines).unwrap();

        assert_eq!(expected, result);
    }
}
