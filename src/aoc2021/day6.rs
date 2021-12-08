fn get_input() -> Vec<u32> {
    include_str!("./inputs/day6.real")
        .trim_end()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect()
}

pub fn solve_one() {
    let input = get_input();
    let mut fishes = [0u32; 9];

    for i in input {
        fishes[i as usize] += 1;
    }

    for _ in 0..80 {
        fishes.rotate_left(1);
        fishes[6] += fishes[8];
        // println!("day: {}", day + 1);
        // println!("{:?}", fishes);
    }

    println!("{:?}", fishes.iter().sum::<u32>());
}

pub fn solve_two() {
    let input = get_input();
    let mut fishes = [0u64; 9];

    for i in input {
        fishes[i as usize] += 1;
    }

    for _ in 0..256 {
        fishes.rotate_left(1);

        fishes[6] += fishes[8];
        // println!("day: {}", day + 1);
        // println!("{:?}", fishes);
    }

    println!("{:?}", fishes.iter().sum::<u64>());
}
