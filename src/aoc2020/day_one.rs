extern crate test;

fn parsed_input() -> Vec<u32> {
    // input is the parsed input file.
    //
    // In this case input is a Vec<u32> where every element represent the year in each line of the
    // input file
    let mut input: Vec<u32> = include_str!("./inputs/day1.txt")
        .trim_end()
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();

    input.sort_unstable();

    input
}

pub fn solve_one() -> Option<u32> {
    let input = parsed_input();
    let mut lpt = 0;
    let mut rpt = input.len() - 1;
    let tar = 2020;

    while lpt < rpt {
        match (input[lpt] + input[rpt]).cmp(&tar) {
            std::cmp::Ordering::Equal => return Some(input[lpt] * input[rpt]),
            std::cmp::Ordering::Less => lpt += 1,
            std::cmp::Ordering::Greater => rpt -= 1,
        }
    }

    None
}

pub fn solve_two() -> Option<u32> {
    let input = parsed_input();
    let mut lpt: usize;
    let mut rpt: usize;
    let mut tar: u32;

    for (i, v) in input.iter().enumerate() {
        tar = 2020 - v;
        lpt = i + 1;
        rpt = input.len() - 1;
        while lpt < rpt {
            match (input[lpt] + input[rpt]).cmp(&tar) {
                std::cmp::Ordering::Equal => return Some(input[lpt] * input[rpt] * v),
                std::cmp::Ordering::Less => lpt += 1,
                std::cmp::Ordering::Greater => rpt -= 1,
            }
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
        assert_eq!(solve_one(), Some(646779));
    }

    #[test]
    fn test_twenty_one_two() {
        assert_eq!(solve_two(), Some(246191688));
    }

    #[bench]
    fn bench_twenty_one_two(b: &mut Bencher) {
        b.iter(solve_one);
    }

    #[bench]
    fn bench_twenty_one_one(b: &mut Bencher) {
        b.iter(solve_two);
    }
}
