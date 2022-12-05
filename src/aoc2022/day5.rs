fn get_input() -> &'static str {
    include_str!("./inputs/day5.real")
}

#[derive(Debug)]
struct Ship {
    stacks: Vec<Vec<char>>,
}

impl Ship {
    pub fn new() -> Self {
        Self { stacks: Vec::new() }
    }

    pub fn stack_to(&mut self, stack: usize, item: char) {
        while stack >= self.stacks.len() {
            self.stacks.push(Vec::new());
        }
        self.stacks[stack].push(item)
    }

    pub fn unstack_from(&mut self, stack: usize) -> Option<char> {
        self.stacks[stack].pop()
    }
}

pub fn solve_one() {
    let (cargo, instructions) = get_input()
        .split_once("\n\n")
        .expect("aoc to be telling the truth");

    let mut ship = Ship::new();

    cargo.lines().rev().skip(1).for_each(|line| {
        line.chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            .filter(|(_, c)| !c.is_whitespace())
            .for_each(|(index, c)| ship.stack_to(index, c));
    });

    instructions
        .lines()
        .map(|l| l.split_whitespace().flat_map(str::parse).collect())
        .for_each(|instruction: Vec<usize>| {
            for _ in 0..instruction[0] {
                let c = ship
                    .unstack_from(instruction[1] - 1)
                    .expect("aoc not to fuck up the crane");

                ship.stack_to(instruction[2] - 1, c)
            }
        });

    let result = ship
        .stacks
        .iter_mut()
        .flat_map(|stack| stack.pop())
        .collect::<String>();

    println!("{}", result)
}

pub fn solve_two() {
    let (cargo, instructions) = get_input()
        .split_once("\n\n")
        .expect("aoc to be telling the truth");

    let mut ship = Ship::new();

    cargo.lines().rev().skip(1).for_each(|line| {
        line.chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            .filter(|(_, c)| !c.is_whitespace())
            .for_each(|(index, c)| ship.stack_to(index, c));
    });

    instructions
        .lines()
        .map(|l| l.split_whitespace().flat_map(|w| w.parse()).collect())
        .for_each(|instruction: Vec<usize>| {
            let mut rev_stack = Vec::with_capacity(instruction[0]);
            for _ in 0..instruction[0] {
                let c = ship
                    .unstack_from(instruction[1] - 1)
                    .expect("aoc not to fuck up the crane");

                rev_stack.push(c);
            }

            rev_stack
                .into_iter()
                .rev()
                .for_each(|c| ship.stack_to(instruction[2] - 1, c));
        });

    let result = ship
        .stacks
        .iter_mut()
        .flat_map(|stack| stack.pop())
        .collect::<String>();

    println!("{}", result)
}
