fn get_input() -> &'static str {
    include_str!("./inputs/day1.real")
}

pub fn solve_one() {
    let max: u32 = get_input()
        .split("\n\n")
        .map(|l| l.trim().lines().flat_map(str::parse::<u32>).sum())
        .max()
        .unwrap();

    println!("{max:?}")
}

pub fn solve_two() {
    let mut calories: Vec<u32> = get_input()
        .split("\n\n")
        .map(|l| l.trim().lines().flat_map(str::parse::<u32>).sum())
        .collect();

    calories.sort_unstable();

    println!("{:?}", calories.iter().rev().take(3).sum::<u32>())
}
