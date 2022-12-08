fn get_input() -> &'static str {
    include_str!("./inputs/day6.real")
}

// answer: 1702
pub fn solve_one() {
    let window_size = 4;
    let chars = get_input()
        .as_bytes()
        .windows(window_size)
        .position(|group| {
            let mut bit_count = 0u32;
            for c in group {
                let copy = bit_count;
                bit_count |= 1 << (c - b'a');
                if copy == bit_count {
                    return false;
                }
            }
            true
        })
        .unwrap();

    println!("result: {:?}", chars + window_size);
}

// answer: 3559
pub fn solve_two() {
    let window_size = 14;
    let chars = get_input()
        .as_bytes()
        .windows(window_size)
        .position(|group| {
            let mut bit_count = 0u32;
            for c in group {
                let copy = bit_count;
                bit_count |= 1 << (c - b'a');
                if copy == bit_count {
                    return false;
                }
            }
            true
        })
        .unwrap();

    println!("result: {:?}", chars + window_size);
}
