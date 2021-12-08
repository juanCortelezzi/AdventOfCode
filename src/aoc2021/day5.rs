use std::{collections::HashMap, num::ParseIntError, ops::RangeInclusive, str::FromStr};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Coord(u32, u32);

impl FromStr for Coord {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').unwrap();

        Ok(Coord(x.parse()?, y.parse()?))
    }
}

#[derive(Debug)]
struct VentLine {
    start: Coord,
    end: Coord,
}

impl FromStr for VentLine {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once(" -> ").unwrap();

        Ok(VentLine {
            start: start.parse()?,
            end: end.parse()?,
        })
    }
}

fn steps(start: u32, end: u32) -> Box<dyn Iterator<Item = u32>> {
    if start < end {
        Box::new(RangeInclusive::new(start, end))
    } else {
        Box::new(RangeInclusive::new(end, start).rev())
    }
}

fn between(start: &Coord, end: &Coord, do_diagonals: bool) -> Vec<Coord> {
    if start.0 == end.0 || start.1 == end.1 {
        // At first this was to hard for me to understand, this is tjdevries code
        //
        // Explanation:
        //
        // start: Coord(0,9) -> end: Coord(5,9)
        //
        // if start.0 == end.0 || start.1 == end.1, steps will return a single item iterator for
        // one of the two relevant calls. The problem is that it will return the matching col OR
        // row of both `Coord`s, but we dont know which. In this case, we know that the call that
        // will return a single item iter will be `steps(start.1, end.1)` because both of them are
        // 9. Somehow we need to map the constant item to the range of the row or column thats
        // moving.
        //
        // `steps(start.0, end.0) -> 0..=5`
        //
        // `steps(start.1, end.1) -> 9..=9`
        //
        // ```
        // 0..=5.flat_map(|x| 9..=9.map(move |9| Coord(x, 9))).collect()
        // ```
        //
        // Now everything should be a bit more clear, notice that in the case where the call
        // `steps(start.0, end.0)` returns a single item iter this example would still give valid
        // answers.
        //
        // ```
        // 9..=9.flat_map(|9| 0..=5.map(move |y| Coord(9, y))).collect()
        // ```
        steps(start.0, end.0)
            .flat_map(|x| steps(start.1, end.1).map(move |y| Coord(x, y)))
            .collect()
    } else if do_diagonals {
        steps(start.0, end.0)
            .zip(steps(start.1, end.1))
            .map(|(x, y)| Coord(x, y))
            .collect()
    } else {
        // We do not care at all!
        Vec::new()
    }
}

fn get_input() -> Vec<VentLine> {
    include_str!("./inputs/day5.real")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

fn get_danger_count(input: &[VentLine], do_diagonals: bool) -> i32 {
    let mut map = HashMap::<Coord, u32>::new();

    for vent in input {
        for coord in between(&vent.start, &vent.end, do_diagonals) {
            map.insert(coord, map.get(&coord).unwrap_or(&0) + 1);
        }
    }

    map.iter()
        .fold(0, |acc, (_, val)| if val > &1 { acc + 1 } else { acc })
}

pub fn solve_one() {
    let input = get_input();
    println!("{}", get_danger_count(&input, false));
}

pub fn solve_two() {
    let input = get_input();
    println!("{}", get_danger_count(&input, true));
}
