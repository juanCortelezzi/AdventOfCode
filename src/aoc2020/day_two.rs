extern crate test;
use nom::{
    bytes::complete::{tag, take_till},
    character::complete::{self, alpha1, anychar, char},
    error::context,
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};

use crate::utils::read_lines;

#[derive(Debug)]
pub struct PassEntry {
    pub min: u32,
    pub max: u32,
    pub letter: char,
    pub password: String,
}

fn parse_passentry(input: &str) -> IResult<&str, PassEntry> {
    context(
        "password",
        tuple((
            terminated(
                separated_pair(complete::u32, char('-'), complete::u32),
                take_till(|c| c == ' '),
            ),
            preceded(tag(" "), anychar),
            preceded(tag(": "), alpha1),
        )),
    )(input)
    .map(|(next_input, res)| {
        let ((min, max), letter, password) = res;
        (
            next_input,
            PassEntry {
                min,
                max,
                letter,
                password: password.to_owned(),
            },
        )
    })
}

fn parsed_input() -> Vec<PassEntry> {
    // input is the parsed input file.
    //
    // In this case input is a Vec<PassEntry> where every element represents a password entry in the
    // database
    read_lines("./src/aoc2020/inputs/day2.txt")
        .expect("Could not read file")
        .map(|l| {
            parse_passentry(&l.unwrap())
                .expect("Could not parse password")
                .1
        })
        .collect()
}

pub fn solve_one() -> u32 {
    let input = parsed_input();
    let mut num_valid_entries = 0;

    for i in input {
        let count = i.password.matches(i.letter).count() as u32;
        if i.min <= count && count <= i.max {
            num_valid_entries += 1;
        }
    }

    num_valid_entries
}

pub fn solve_two() -> u32 {
    let input = parsed_input();
    let mut num_valid_entries = 0;

    for i in input {
        let mut chars = i.password.chars();
        let mut valid = false;

        if chars.nth((i.min - 1) as usize) == Some(i.letter) {
            valid = !valid;
        }

        if chars.nth((i.max - i.min - 1) as usize) == Some(i.letter) {
            valid = !valid;
        }

        if valid {
            num_valid_entries += 1;
        }
    }

    num_valid_entries
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_twenty_two_one() {
        assert_eq!(solve_one(), 439);
    }

    #[test]
    fn test_twenty_two_two() {
        assert_eq!(solve_two(), 584);
    }

    #[bench]
    fn bench_twenty_two_one(b: &mut Bencher) {
        b.iter(solve_one);
    }

    #[bench]
    fn bench_twenty_two_two(b: &mut Bencher) {
        b.iter(solve_two);
    }
}
