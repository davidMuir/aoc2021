use std::io::{stdin, BufRead};

use clap::{App, Arg};

use crate::{day1::solve_day_1, day2::solve_day_2};

mod day1;
mod day2;

fn main() {
    let matches = App::new("aoc2021")
        .version("1.0")
        .author("David Muir")
        .arg(
            Arg::with_name("day")
                .long("day")
                .short("d")
                .takes_value(true),
        )
        .get_matches();

    match matches.value_of("day") {
        Some("1") => {
            let count = solve_day_1(stdin().lock().lines());

            println!("{} increases", count);
        }
        Some("2") => {
            let answer = solve_day_2(stdin().lock().lines());

            println!("Answer: {}", answer);
        }
        Some(day) => {
            println!("No solution found for day {}", day);
        }
        _ => {
            println!("No solution found");
        }
    }
}
