fn parse_input(line: &str) -> Vec<usize> {
    line.chars()
        .map(|c| match c {
            '1' => 1,
            '0' => 0,
            _ => panic!("failed to parse input"),
        })
        .collect()
}

fn get_input() -> Vec<Vec<usize>> {
    include_str!("./inputs/day3.real")
        .trim_end()
        .split('\n')
        .map(parse_input)
        .collect()
}

pub fn solve_one() {
    let input = get_input();
    let mut gamma = String::with_capacity(input[0].len());
    let mut epsilon = String::with_capacity(input[0].len());

    for i in 0..input[0].len() {
        let one_amount = input.iter().fold(0, |acc, x| acc + x[i]);
        let zero_amount = input.len() - one_amount;
        gamma.push(if one_amount > zero_amount { '1' } else { '0' });
        epsilon.push(if one_amount > zero_amount { '0' } else { '1' });
    }

    let gamma = i32::from_str_radix(&gamma, 2).unwrap();
    let epsilon = i32::from_str_radix(&epsilon, 2).unwrap();

    println!("res: {}", gamma * epsilon);
}

pub fn solve_two() {
    let input = get_input();
    let oxy = get_oxygen_rating(input.clone());
    let co2 = get_co2_rating(input);

    println!("{} - {}", oxy, co2);
    println!("res: {}", oxy * co2);
}

fn get_oxygen_rating(mut input: Vec<Vec<usize>>) -> i32 {
    let mut col = 0;
    while input.len() > 1 {
        let one_amount = input.iter().fold(0, |acc, x| acc + x[col]);
        let zero_amount = input.len() - one_amount;

        input = input
            .into_iter()
            .filter(|x| x[col] == if one_amount >= zero_amount { 1 } else { 0 })
            .collect::<Vec<_>>();

        col += 1;
    }

    binarray_to_dec(&input[0])
}

fn get_co2_rating(mut input: Vec<Vec<usize>>) -> i32 {
    let mut col = 0;
    while input.len() > 1 {
        let one_amount = input.iter().fold(0, |acc, x| acc + x[col]);
        let zero_amount = input.len() - one_amount;

        input = input
            .into_iter()
            .filter(|x| x[col] == if one_amount >= zero_amount { 0 } else { 1 })
            .collect::<Vec<_>>();

        col += 1;
    }

    println!("{:?}", input[0]);
    binarray_to_dec(&input[0])
}

fn binarray_to_dec(input: &[usize]) -> i32 {
    let mut sum = 0;
    for (i, v) in input.iter().rev().enumerate() {
        if v == &1 {
            sum += 2_i32.pow(i as u32);
        }
    }

    sum
}
