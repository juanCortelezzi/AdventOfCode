use anyhow::{anyhow, Result};

fn get_input() -> &'static str {
    include_str!("./inputs/day2.real")
}

fn value_to_number(x: &'static str) -> Result<u32> {
    match x {
        "A" | "X" => Ok(1),
        "B" | "Y" => Ok(2),
        "C" | "Z" => Ok(3),
        _ => Err(anyhow!("not a valid rps char")),
    }
}

pub fn solve_one() {
    let x = get_input()
        .lines()
        .flat_map(|line| line.split_once(' '))
        .map(|(a, b)| (value_to_number(a).unwrap(), value_to_number(b).unwrap()))
        .map(|(a, b)| {
            let b_demise = if b + 1 > 3 { 1 } else { b + 1 };
            println!("{a} - {b} | {b_demise}");
            if a == b_demise {
                b
            } else if a == b {
                b + 3
            } else {
                b + 6
            }
        })
        .sum::<u32>();

    println!("{:?}", x);
}

pub fn solve_two() {
    let x = get_input()
        .lines()
        .flat_map(|line| line.split_once(' '))
        .map(|(a, b)| {
            let a = value_to_number(a).unwrap();
            match b {
                "X" => {
                    if a - 1 < 1 {
                        3
                    } else {
                        a - 1
                    }
                }
                "Y" => a + 3,
                "Z" => {
                    let a_demise = if a + 1 > 3 { 1 } else { a + 1 };
                    a_demise + 6
                }
                _ => panic!("not valid b value"),
            }
        })
        .sum::<u32>();

    println!("{:?}", x);
}
