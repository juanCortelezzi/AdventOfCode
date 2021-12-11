fn is_chunk_opener(c: char) -> bool {
    ['(', '[', '{', '<'].contains(&c)
}

fn reverse(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        ')' => ')',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        _ => panic!("`c` is not a valid pair character"),
    }
}

fn get_input() -> Vec<Vec<char>> {
    include_str!("./inputs/day10.real")
        .lines()
        .map(|s| s.chars().collect())
        .collect()
}

pub fn solve_one() {
    let input = get_input();
    let mut stack: Vec<char> = Vec::new();
    let mut failed: Vec<char> = Vec::new();

    for entry in input {
        for c in entry.into_iter() {
            let last = stack.iter().last();

            if is_chunk_opener(c) {
                stack.push(c);
                continue;
            }

            if last.is_none() || reverse(*last.unwrap()) != c {
                failed.push(c);
                break;
            }

            stack.pop();
        }
        stack.clear();
    }

    let res: u32 = failed
        .iter()
        .map(|&c| match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => panic!("`c` is not a valid closing pair"),
        })
        .sum();

    println!("res: {}", res);
}

pub fn solve_two() {
    let input = get_input();
    let mut stack: Vec<char> = Vec::with_capacity(12);
    let mut scores = Vec::new();

    for entry in input {
        for &c in entry.iter() {
            let last = stack.iter().last();
            if is_chunk_opener(c) {
                stack.push(c);
                continue;
            }

            if last.is_none() || reverse(*last.unwrap()) != c {
                stack.clear();
                break;
            }

            stack.pop();
        }

        if stack.is_empty() {
            continue;
        }

        let score = stack
            .iter()
            .rev()
            .map(|&c| reverse(c))
            .fold(0u64, |acc, c| {
                acc * 5
                    + match c {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => panic!("`c` is not a valid closing pair"),
                    }
            });

        scores.push(score);

        stack.clear();
    }

    scores.sort_unstable();

    println!("{:?}", scores[scores.len() / 2]);
}
