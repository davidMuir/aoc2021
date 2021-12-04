enum Direction {
    Up,
    Down,
    Forward,
    Invalid,
}

pub fn solve_day_2(lines: impl Iterator<Item = Result<String, std::io::Error>>) -> i32 {
    let x = lines
        .flatten()
        .map(|l| {
            let mut iter = l.split_whitespace();

            let direction = iter.next().unwrap();
            let amount = iter.next().unwrap().parse::<i32>().unwrap();

            let direction = match direction {
                "forward" => Direction::Forward,
                "down" => Direction::Down,
                "up" => Direction::Up,
                _ => Direction::Invalid,
            };

            (direction, amount)
        })
        .fold((0, 0, 0), |a, c| match c {
            (Direction::Forward, amount) => (a.0 + amount, a.1 + amount * a.2, a.2),
            (Direction::Down, amount) => (a.0, a.1, a.2 + amount),
            (Direction::Up, amount) => (a.0, a.1, a.2 - amount),
            _ => a,
        });

    x.0 * x.1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_data() {
        let lines = vec![
            Ok("forward 5".to_string()),
            Ok("down 5".to_string()),
            Ok("forward 8".to_string()),
            Ok("up 3".to_string()),
            Ok("down 8".to_string()),
            Ok("forward 2".to_string()),
        ]
        .into_iter();

        let expected = 900;
        let result = solve_day_2(lines);

        assert_eq!(expected, result);
    }
}
