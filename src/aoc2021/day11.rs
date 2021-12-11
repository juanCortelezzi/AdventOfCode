struct Squid {
    flashed: bool,
    value: u32,
}

impl Squid {
    fn new(value: u32) -> Self {
        Squid {
            flashed: false,
            value,
        }
    }
}

struct Map {
    width: i32,
    height: i32,
    cells: Vec<Vec<Squid>>,
}

impl Map {
    const DIRS: [(i32, i32); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    fn in_bounds(&self, row: i32, col: i32) -> bool {
        row >= 0 && row < self.height as i32 && col >= 0 && col < self.width as i32
    }

    fn able_to_flash(&self, row: i32, col: i32) -> bool {
        self.in_bounds(row, col)
            && self.cells[row as usize][col as usize].value > 9
            && !self.cells[row as usize][col as usize].flashed
    }

    fn flash(&mut self, row: i32, col: i32) -> u32 {
        if !self.able_to_flash(row, col) {
            return 0;
        }

        self.cells[row as usize][col as usize].flashed = true;

        let dirs: Vec<(i32, i32)> = Self::DIRS
            .iter()
            .map(|dir| (dir.0 + row, dir.1 + col))
            .filter(|dir| self.in_bounds(dir.0, dir.1))
            .collect();

        for &(row, col) in &dirs {
            self.cells[row as usize][col as usize].value += 1;
        }

        dirs.iter().fold(1, |acc, &(row, col)| {
            if self.able_to_flash(row, col) {
                acc + self.flash(row, col)
            } else {
                acc
            }
        })
    }

    fn step(&mut self) -> u32 {
        for entry in &mut self.cells {
            for squid in entry {
                squid.value += 1;
            }
        }

        let mut sum = 0;
        for r in 0..self.height {
            for c in 0..self.width {
                sum += self.flash(r, c);
            }
        }

        for entry in &mut self.cells {
            entry
                .iter_mut()
                .filter(|squid| squid.flashed)
                .for_each(|squid| {
                    squid.flashed = false;
                    squid.value = 0;
                })
        }

        sum
    }
}

fn get_input() -> Map {
    let cells: Vec<Vec<Squid>> = include_str!("./inputs/day11.real")
        .lines()
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).unwrap())
                .map(Squid::new)
                .collect()
        })
        .collect();

    Map {
        width: 10,
        height: 10,
        cells,
    }
}

pub fn solve_one() {
    let mut map = get_input();

    let res: u32 = (0..100).map(|_| map.step()).sum();
    println!("{}", res);
}

pub fn solve_two() {
    let mut map = get_input();

    let mut iter = 1;
    while map.step() != 100 {
        iter += 1;
    }
    println!("{}", iter);
}
