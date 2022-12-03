fn get_input() -> Vec<Vec<char>> {
    include_str!("./inputs/day3.real")
        .trim()
        .lines()
        .map(|l| l.chars().collect())
        .collect()
}

fn solve_slope(input: &Vec<Vec<char>>, slope: (usize, usize)) -> u32 {
    let map_height = input.len();
    let map_width = input.get(0).unwrap().len();
    let mut tree_count = 0;
    let mut position = (0, 0);
    while position.1 != map_height - 1 {
        position = (position.0 + slope.0, position.1 + slope.1);
        let character = input[position.1 % map_height][position.0 % map_width];
        if character == '#' {
            tree_count += 1;
        }
    }
    tree_count
}

pub fn solve_one() {
    let input = get_input();
    let result = solve_slope(&input, (3, 1));
    println!("result: {}", result)
}
pub fn solve_two() {
    let input = get_input();
    let result = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .into_iter()
        .map(|slope| solve_slope(&input, slope))
        .reduce(|acc, e| acc * e);
    println!("result: {}", result.unwrap())
}
