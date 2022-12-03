use std::collections::HashSet;

fn get_input() -> &'static str {
    include_str!("./inputs/day3.real")
}

const CHARS: [char; 52] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

pub fn solve_one() {
    let mut hashset = std::collections::HashSet::<char>::new();
    let x = get_input()
        .lines()
        .map(|sacks| sacks.split_at(sacks.len() / 2))
        .map(|(left, right)| {
            left.chars().collect_into(&mut hashset);

            let right = right
                .chars()
                .flat_map(|c| hashset.contains(&c).then_some(c))
                .last()
                .unwrap();

            hashset.clear();

            right
        })
        .flat_map(|c| CHARS.iter().position(|letter| letter == &c).map(|c| c + 1))
        .sum::<usize>();
    println!("{x}")
}

pub fn solve_two() {
    let sacks = get_input().lines().collect::<Vec<&'static str>>();
    let x = sacks
        .array_chunks::<3>()
        .map(|sacks| {
            let mut hashmap = std::collections::HashMap::<char, usize>::new();
            for c in sacks
                .iter()
                .flat_map(|sack| sack.chars().collect::<HashSet<char>>())
            {
                hashmap.entry(c).and_modify(|x| *x += 1).or_insert(1);
            }

            hashmap.into_iter().find(|(_, q)| *q == 3).unwrap()
        })
        .flat_map(|x| {
            CHARS
                .iter()
                .position(|letter| letter == &x.0)
                .map(|c| c + 1)
        })
        .sum::<usize>();

    println!("{x}")
}
