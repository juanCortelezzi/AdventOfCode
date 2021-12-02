fn get_input() -> Vec<i32> {
    include_str!("./inputs/day1.real")
        .trim_end()
        .split('\n')
        .filter_map(|x| x.parse::<i32>().ok())
        .collect()
}

pub fn solve_one() {
    let data = get_input()
        .windows(2)
        .fold(0, |acc, win| if win[1] > win[0] { acc + 1 } else { acc });

    println!("{}", data);
}

pub fn solve_two() {
    let data = get_input().windows(4).fold(0, |acc, win| {
        if win[0..3].iter().sum::<i32>() < win[1..4].iter().sum::<i32>() {
            acc + 1
        } else {
            acc
        }
    });

    println!("{}", data);
}
