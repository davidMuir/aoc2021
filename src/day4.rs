pub fn solve_day_4(lines: impl Iterator<Item = Result<String, std::io::Error>>) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_data() {
        let lines = vec![
            Ok(
                "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1"
                    .to_string(),
            ),
            Ok("".to_string()),
            Ok("22 13 17 11  0".to_string()),
            Ok(" 8  2 23  4 24".to_string()),
            Ok("21  9 14 16  7".to_string()),
            Ok(" 6 10  3 18  5".to_string()),
            Ok(" 1 12 20 15 19".to_string()),
            Ok("".to_string()),
            Ok(" 3 15  0  2 22".to_string()),
            Ok(" 9 18 13 17  5".to_string()),
            Ok("19  8  7 25 23".to_string()),
            Ok("20 11 10 24  4".to_string()),
            Ok("14 21 16 12  6".to_string()),
            Ok("".to_string()),
            Ok("14 21 17 24  4".to_string()),
            Ok("10 16 15  9 19".to_string()),
            Ok("18  8 23 26 20".to_string()),
            Ok("22 11 13  6  5".to_string()),
            Ok(" 2  0 12  3  7".to_string()),
        ]
        .into_iter();

        let expected = 4512;
        let result = solve_day_4(lines);

        assert_eq!(expected, result);
    }
}
