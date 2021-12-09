fn get_fuel_consumption(crabs: &[i32], target: i32) -> i32 {
    crabs.iter().fold(0, |acc, x| acc + (x - target).abs())
}

fn get_fuel_consumption_complex(crabs: &[i32], target: i32) -> i32 {
    crabs.iter().fold(0, |acc, x| {
        let diff = (x - target).abs();
        acc + diff * (diff + 1) / 2
    })
}

fn get_input() -> Vec<i32> {
    include_str!("./inputs/day7.real")
        .trim_end()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect()
}

pub fn solve_one() {
    let mut crabs = get_input();

    crabs.sort_unstable();

    let middle = crabs[crabs.len() / 2];

    println!("res: {}", get_fuel_consumption(&crabs, middle));
}

pub fn solve_two() {
    let crabs = get_input();
    // Gonna brute force it because I am not that smart of a guy :)

    let min = crabs.iter().min().unwrap();
    let max = crabs.iter().max().unwrap();

    println!("min: {} - max: {}", min, max);

    let res = (*min..*max)
        .map(|t| get_fuel_consumption_complex(&crabs, t))
        .min()
        .unwrap();

    println!("{}", res);
}
