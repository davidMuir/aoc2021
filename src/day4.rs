pub fn solve_day_4(lines: impl Iterator<Item = Result<String, std::io::Error>>) -> u32 {
    let lines = lines.flatten().collect::<Vec<String>>();

    let draws = get_draws(&lines);
    let boards = get_boards(&lines);

    let winning_board = boards
        .iter()
        .map(|b| b.get_win(&draws))
        .filter(|r| matches!(r, Some(_)))
        .flatten()
        .max_by(|a, b| a.0.cmp(&b.0))
        .unwrap();

    winning_board.1
}

fn get_draws(lines: &[String]) -> Vec<u32> {
    let line = lines.first().unwrap();

    line.split(',').flat_map(|l| l.parse::<u32>()).collect()
}

fn get_boards(lines: &[String]) -> Vec<Board> {
    let numbers = lines
        .iter()
        .skip(2)
        .filter(|l| !l.is_empty())
        .flat_map(|l| {
            l.split_whitespace()
                .into_iter()
                .flat_map(|l| l.parse::<u32>())
        })
        .collect::<Vec<u32>>();

    let mut boards = vec![];
    let mut i = 0;

    loop {
        let numbers = numbers
            .iter()
            .skip(i)
            .take(25)
            .map(|n| n.to_owned())
            .collect::<Vec<u32>>();

        if numbers.len() < 25 {
            return boards;
        }

        let board = Board {
            numbers: numbers.try_into().unwrap(),
        };

        boards.push(board);
        i += 25;
    }
}

#[derive(Debug, PartialEq)]
struct Board {
    numbers: [u32; 25],
}

impl Board {
    fn get_win(&self, draw: &[u32]) -> Option<(usize, u32)> {
        for n in 0..draw.len() {
            let numbers = draw
                .iter()
                .take(n)
                .map(|n| n.to_owned())
                .collect::<Vec<u32>>();

            let result = self.get_score(numbers);

            if let Some(r) = result {
                return Some((n, r));
            }
        }

        None
    }

    fn get_score(&self, draw: Vec<u32>) -> Option<u32> {
        let has_matching_rows = self
            .numbers
            .chunks(5)
            .any(|c| c.iter().all(|n| draw.contains(n)));
        let has_matching_cols = (0..5)
            .map(|col| {
                self.numbers
                    .iter()
                    .enumerate()
                    .filter(move |(i, _n)| i % 5 == col)
                    .map(|(_i, n)| n)
                    .take(5)
            })
            .any(|mut col| col.all(|n| draw.contains(n)));

        if has_matching_cols || has_matching_rows {
            Some(
                self.numbers
                    .iter()
                    .filter(|n| !draw.contains(n))
                    .map(|n| n.to_owned())
                    .sum::<u32>()
                    * draw.last().unwrap(),
            )
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_sample_data() -> Vec<Result<String, std::io::Error>> {
        vec![
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
    }

    #[test]
    fn sample_data() {
        let lines = get_sample_data().into_iter();

        let expected = 1924;
        let result = solve_day_4(lines);

        assert_eq!(expected, result);
    }

    #[test]
    fn gets_draws() {
        let lines = get_sample_data()
            .into_iter()
            .flatten()
            .collect::<Vec<String>>();

        let expected = vec![
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ];
        let result = get_draws(&lines);

        assert_eq!(expected, result);
    }

    #[test]
    fn gets_boards() {
        let lines = get_sample_data()
            .into_iter()
            .flatten()
            .collect::<Vec<String>>();

        let expected = vec![
            Board {
                numbers: [
                    22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12,
                    20, 15, 19,
                ],
            },
            Board {
                numbers: [
                    3, 15, 0, 2, 22, 9, 18, 13, 17, 5, 19, 8, 7, 25, 23, 20, 11, 10, 24, 4, 14, 21,
                    16, 12, 6,
                ],
            },
            Board {
                numbers: [
                    14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5, 2,
                    0, 12, 3, 7,
                ],
            },
        ];
        let result = get_boards(&lines);

        assert_eq!(expected, result);
    }

    #[test]
    fn board_gets_win() {
        let board = Board {
            numbers: [
                14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5, 2, 0,
                12, 3, 7,
            ],
        };
        let draw = vec![
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ];

        let expected = Some((12, 4512));
        let result = board.get_win(&draw);

        assert_eq!(expected, result);
    }
}
