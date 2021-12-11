const DIRS: [(i32, i32); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];

struct Map {
    height: i32,
    width: i32,
    cells: Vec<u8>,
}

impl Map {
    fn get_index(&self, row: i32, column: i32) -> usize {
        (row * self.width + column) as usize
    }

    fn in_bounds(&self, row: i32, col: i32) -> bool {
        row >= 0 && row < self.height && col >= 0 && col < self.width
    }

    fn get_neighbors(&self, row: i32, col: i32) -> Vec<u8> {
        DIRS.iter()
            .filter(|dir| self.in_bounds(dir.0 + row, dir.1 + col))
            .map(|dir| self.cells[self.get_index(dir.0 + row, dir.1 + col)])
            .collect()
    }

    fn is_walkable(&self, row: i32, col: i32) -> bool {
        self.in_bounds(row, col) && self.cells[self.get_index(row, col)] != 9
    }

    fn get_basin(&mut self, row: i32, col: i32) -> u32 {
        if !self.is_walkable(row, col) {
            return 0;
        }

        self.cells[(row * self.width + col) as usize] = 9;

        DIRS.iter()
            .fold(0, |acc, (r, c)| acc + self.get_basin(*r + row, *c + col))
            + 1
    }

    fn is_low_point(&self, row: i32, col: i32) -> bool {
        !self
            .get_neighbors(row, col)
            .iter()
            .any(|&x| x <= self.cells[self.get_index(row, col)])
    }
}

fn get_input() -> Map {
    let input: Vec<Vec<u8>> = include_str!("./inputs/day9.real")
        .lines()
        .map(|s| s.chars().map(|c| c.to_string().parse().unwrap()).collect())
        .collect();

    let height = input.len() as i32;
    let width = input[0].len() as i32;

    Map {
        height,
        width,
        cells: input.into_iter().flatten().collect(),
    }
}

pub fn solve_one() {
    let map = get_input();

    let mut low_points: Vec<u8> = Vec::new();
    for r in 0..map.height {
        for c in 0..map.width {
            if map.is_low_point(r, c) {
                low_points.push(map.cells[map.get_index(r, c)]);
            }
        }
    }

    let res = low_points.iter().fold(0u32, |acc, x| acc + *x as u32 + 1);
    println!("{}", res);
}

pub fn solve_two() {
    let mut map = get_input();

    let mut low_points: Vec<(i32, i32)> = Vec::new();
    for r in 0..map.height {
        for c in 0..map.width {
            if map.is_low_point(r, c) {
                low_points.push((r, c));
            }
        }
    }

    let mut basins = Vec::with_capacity(low_points.len());
    for i in low_points {
        basins.push(map.get_basin(i.0, i.1))
    }

    basins.sort_unstable();

    println!("res: {}", basins.iter().rev().take(3).product::<u32>());
}
