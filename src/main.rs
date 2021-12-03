use std::io::{stdin, BufRead};

use crate::day1::solve_day_1;

mod day1;

fn main() {
    let count = solve_day_1(stdin().lock().lines());

    println!("{} increases", count);
}
