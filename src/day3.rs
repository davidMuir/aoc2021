use std::error::Error;

pub fn solve_day_3(
    lines: impl Iterator<Item = Result<String, std::io::Error>>,
) -> Result<u32, Box<dyn Error>> {
    let base: u32 = 2;
    let (gamma, epsilon) = lines
        .flatten()
        .map(|l| l.chars().map(to_bool).collect::<Vec<bool>>())
        .fold(None::<Vec<(i32, i32)>>, |a, c| {
            if let Some(a) = a {
                Some(
                    a.iter()
                        .zip(c)
                        .map(|f| {
                            if f.1 {
                                (f.0 .0, f.0 .1 + 1)
                            } else {
                                (f.0 .0 + 1, f.0 .1)
                            }
                        })
                        .collect(),
                )
            } else {
                Some(c.iter().map(|x| if *x { (0, 1) } else { (1, 0) }).collect())
            }
        })
        .unwrap()
        .iter()
        .rev()
        .enumerate()
        .fold((0, 0), |a, (i, c)| {
            if c.1 > c.0 {
                (a.0 + base.pow(i as u32), a.1)
            } else {
                (a.0, a.1 + base.pow(i as u32))
            }
        });

    Ok(gamma * epsilon)
}

fn to_bool(c: char) -> bool {
    c == '1'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_data() {
        let lines = vec![
            Ok("00100".to_string()),
            Ok("11110".to_string()),
            Ok("10110".to_string()),
            Ok("10111".to_string()),
            Ok("10101".to_string()),
            Ok("01111".to_string()),
            Ok("00111".to_string()),
            Ok("11100".to_string()),
            Ok("10000".to_string()),
            Ok("11001".to_string()),
            Ok("00010".to_string()),
            Ok("01010".to_string()),
        ]
        .into_iter();

        let expected = 198;
        let result = solve_day_3(lines).unwrap();

        assert_eq!(expected, result);
    }
}
