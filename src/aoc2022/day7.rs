// This one was the first "tough" one.
// Also, I hate this code

fn get_input() -> &'static str {
    include_str!("./inputs/day7.real")
}

fn cleaner(stack: &mut Vec<(&'static str, usize)>, map: &mut Vec<(&'static str, usize)>) {
    let (dir_name, dir_size) = stack.pop().unwrap();

    map.push((dir_name, dir_size));

    if let Some(last) = stack.last_mut() {
        last.1 += dir_size;
    };
}

pub fn solve_one() {
    let mut map = Vec::<(&'static str, usize)>::new();
    let mut stack = vec![("/", 0)];

    for instruction in get_input()
        .lines()
        .filter(|&line| line != "$ cd /" && line != "$ ls")
    {
        println!("{:?}", instruction);

        if let Some((_, dir)) = instruction.split_once("$ cd ") {
            if dir == ".." {
                cleaner(&mut stack, &mut map);
                continue;
            };
            stack.push((dir, 0));
            continue;
        }

        let (a, _) = instruction
            .split_once(' ')
            .expect("aoc not to be fucking with me");

        if let Ok(a) = a.parse::<usize>() {
            stack.last_mut().unwrap().1 += a;
        }
    }

    for _ in 0..stack.len() {
        cleaner(&mut stack, &mut map);
    }

    let x = map
        .iter()
        .map(|x| x.1)
        .filter(|&x| x <= 100_000)
        .sum::<usize>();

    println!("{x}")
}

pub fn solve_two() {
    let total_space = 70_000_000;
    let desired_space = 30_000_000;

    let mut map = Vec::<(&'static str, usize)>::new();
    let mut stack = vec![("/", 0)];

    for instruction in get_input()
        .lines()
        .filter(|&line| line != "$ cd /" && line != "$ ls")
    {
        println!("{:?}", instruction);

        if let Some((_, dir)) = instruction.split_once("$ cd ") {
            if dir == ".." {
                cleaner(&mut stack, &mut map);
                continue;
            };
            stack.push((dir, 0));
            continue;
        }

        let (a, _) = instruction
            .split_once(' ')
            .expect("aoc not to be fucking with me");

        if let Ok(a) = a.parse::<usize>() {
            stack.last_mut().unwrap().1 += a;
        }
    }

    for _ in 0..stack.len() {
        cleaner(&mut stack, &mut map);
    }

    let root_size = map.iter().map(|x| x.1).max().unwrap();
    let remaining_space = total_space - root_size;
    let desired_space = desired_space - remaining_space;

    let x = map
        .iter()
        .map(|x| x.1)
        .filter(|&x| x >= desired_space)
        .min()
        .unwrap();

    println!("{x}")
}
