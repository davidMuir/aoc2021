use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::day1::solve_day_1;

mod day1;

fn main() -> Result<(), Box<dyn Error>> {
    let input = File::open("/home/david/src/personal/projects/aoc2021/inputs/day1.txt")?;
    let buffered = BufReader::new(input);

    let count = solve_day_1(buffered.lines())?;

    println!("{} increases", count);

    Ok(())
}
