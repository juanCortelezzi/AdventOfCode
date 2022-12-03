fn get_input() -> Vec<u32> {
    let mut arr: Vec<u32> = include_str!("./inputs/day1.real")
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();

    arr.sort_unstable();
    arr
}

pub fn solve_one() {
    let input = get_input();
    let mut lp = 0;
    let mut rp = input.len() - 1;
    while lp < rp {
        match (input[lp] + input[rp]).cmp(&2020) {
            std::cmp::Ordering::Less => lp += 1,
            std::cmp::Ordering::Equal => break,
            std::cmp::Ordering::Greater => rp -= 1,
        }
    }
    println!("{}", input[lp] * input[rp])
}

pub fn solve_two() {
    let input = get_input();
    'falopa: for number in input.iter() {
        let target = 2020 - number;
        let mut lp = 0;
        let mut rp = input.len() - 1;
        while lp < rp {
            match (input[lp] + input[rp]).cmp(&target) {
                std::cmp::Ordering::Less => lp += 1,
                std::cmp::Ordering::Equal => {
                    println!("{}", input[lp] * input[rp] * number);
                    break 'falopa;
                }
                std::cmp::Ordering::Greater => rp -= 1,
            }
        }
    }
}
