extern crate test;

use crate::utils::read_lines;

pub fn solve() -> Option<u32> {
    let input: Vec<u32> =
        read_lines("/home/wiz/Documents/Rust-programes/puzzles/src/aoc/inputs/2020-1-1.txt")
            .expect("Could not read file")
            .map(|l| l.unwrap().parse::<u32>().unwrap())
            .collect();

    for x in input.iter() {
        if input.iter().any(|&y| y == 2020 - x) {
            return Some(x * (2020 - x));
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_twenty_one_one() {
        assert_eq!(solve(), Some(646779));
    }

    #[bench]
    fn bench_twenty_one_one(b: &mut Bencher) {
        b.iter(solve);
    }
}
