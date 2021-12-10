// I watched thePrimeagens stream where he was doing this day before I did. Could not held myself
// back to not use this beautiful projection. In addition ... Bit tickling is the best
fn projector(digit: &str) -> u8 {
    // char -> bit-flag
    // 0000000
    // gfedcba
    digit.chars().map(|c| 0x1 << (c as u8 - b'a')).sum::<u8>()
}

fn contains(x: u8, y: u8) -> bool {
    (x ^ y) & y == 0
}

fn missing_one(x: u8, y: u8) -> bool {
    ((x ^ y) & y).count_ones() == 1
}

fn missing_two(x: u8, y: u8) -> bool {
    ((x ^ y) & y).count_ones() == 2
}

fn filter(i: &[u8]) -> [u8; 10] {
    let mut found = [0u8; 10];

    found[1] = *i.iter().take(10).find(|x| x.count_ones() == 2).unwrap();
    found[4] = *i.iter().take(10).find(|x| x.count_ones() == 4).unwrap();
    found[7] = *i.iter().take(10).find(|x| x.count_ones() == 3).unwrap();
    found[8] = 0b1111111;

    found[9] = *i
        .iter()
        .take(10)
        .find(|x| contains(**x, found[4]) && x.count_ones() == 6)
        .unwrap();
    found[2] = *i
        .iter()
        .take(10)
        .find(|x| x.count_ones() == 5 && missing_two(**x, found[9]))
        .unwrap();
    found[3] = *i
        .iter()
        .take(10)
        .find(|x| x.count_ones() == 5 && contains(**x, found[1]))
        .unwrap();
    found[5] = *i
        .iter()
        .take(10)
        .find(|x| x.count_ones() == 5 && missing_one(**x, found[9]) && !contains(**x, found[1]))
        .unwrap();
    found[6] = *i
        .iter()
        .take(10)
        .find(|x| x.count_ones() == 6 && contains(**x, found[5]) && missing_one(**x, found[4]))
        .unwrap();
    found[0] = *i.iter().take(10).find(|x| !found.contains(x)).unwrap();

    found
}

fn parse_input(io: (&str, &str)) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::with_capacity(14);
    io.0.split(' ').map(projector).for_each(|n| out.push(n));
    io.1.split(' ').map(projector).for_each(|n| out.push(n));
    out
}

fn get_input() -> Vec<Vec<u8>> {
    include_str!("./inputs/day8.test")
        .lines()
        .map(|s| s.split_once(" | ").unwrap())
        .map(parse_input)
        .collect()
}

pub fn solve_one() {
    let input = get_input();

    let count = input.iter().fold(0, |mut acc, r| {
        for flags in r.iter().skip(10) {
            if [2, 4, 3, 7].contains(&flags.count_ones()) {
                acc += 1;
            }
        }
        acc
    });

    println!("{}", count);
}

pub fn solve_two() {
    let input = get_input();

    let count = input.iter().fold(0, |acc, i| {
        let found = filter(i);

        acc + i
            .iter()
            .skip(10)
            .map(|x| found.iter().position(|&f| f == *x).unwrap())
            .rev()
            .enumerate()
            .fold(0, |acc, (i, x)| acc + x * (10_usize.pow(i as u32)))
    });

    println!("{}", count);
}
