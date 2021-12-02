#[derive(Debug)]
enum Direction {
    Forward(usize),
    Up(usize),
    Down(usize),
}

struct Position {
    horizontal: usize,
    depth: usize,
    aim: usize,
}

impl Position {
    fn new() -> Self {
        Position {
            horizontal: 0,
            depth: 0,
            aim: 0,
        }
    }

    fn result(&self) -> usize {
        self.horizontal * self.depth
    }
}

fn parse_line(line: &str) -> Option<Direction> {
    match line.split_once(' ') {
        Some((direction, amount)) => {
            let amount = amount.parse().unwrap();

            match direction {
                "forward" => Some(Direction::Forward(amount)),
                "up" => Some(Direction::Up(amount)),
                "down" => Some(Direction::Down(amount)),
                _ => panic!("could not parse"),
            }
        }
        None => None,
    }
}

fn get_input() -> Vec<Direction> {
    include_str!("./inputs/day2.test")
        .trim_end()
        .split('\n')
        .filter_map(parse_line)
        .collect::<Vec<Direction>>()
}

pub fn solve_one() {
    let mut pos = Position::new();

    for dir in get_input() {
        match dir {
            Direction::Forward(x) => {
                pos.horizontal += x;
                pos.depth += pos.aim * x
            }
            Direction::Up(x) => pos.aim -= x,
            Direction::Down(x) => pos.aim += x,
        }
    }

    println!(
        "hor: {}\ndepth: {}\nres: {}",
        pos.horizontal,
        pos.depth,
        pos.result()
    )
}

pub fn solve_two() {}
