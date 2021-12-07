#![allow(dead_code)]

#[derive(Debug, Copy, Clone)]
enum BoardSlot {
    Marked(i32),
    UnMarked(i32),
}

#[derive(Debug)]
struct Board {
    board: [[BoardSlot; 5]; 5],
}

impl Board {
    fn from_strings(lines: &[String]) -> Self {
        let mut board = Board {
            board: [[BoardSlot::UnMarked(0); 5]; 5],
        };

        lines
            .iter()
            .map(|s| s.split_whitespace().filter(|s| !s.is_empty()))
            .flatten()
            .map(|s| s.parse().expect("could not parse String to u32"))
            .enumerate()
            .for_each(|(i, x)| {
                board.board[i / 5][i % 5] = BoardSlot::UnMarked(x);
            });

        board
    }

    fn mark(&mut self, number: i32) -> bool {
        for r in 0..5 {
            for c in 0..5 {
                if let BoardSlot::UnMarked(x) = self.board[r][c] {
                    if x == number {
                        self.board[r][c] = BoardSlot::Marked(number);
                        return self.check_win(c, r);
                    }
                }
            }
        }

        false
    }

    fn check_win(&self, col: usize, row: usize) -> bool {
        let mut col_win = true;
        for r in 0..5 {
            if let BoardSlot::UnMarked(_) = self.board[r][col] {
                col_win = false;
            }
        }

        if col_win {
            return true;
        }

        let mut row_win = true;
        for c in 0..5 {
            if let BoardSlot::UnMarked(_) = self.board[row][c] {
                row_win = false;
            }
        }

        row_win
    }

    fn sum(&self) -> i32 {
        self.board.iter().fold(0, |acc, x| {
            let suma: i32 = x
                .iter()
                .filter(|slot| match slot {
                    BoardSlot::Marked(_) => false,
                    BoardSlot::UnMarked(_) => true,
                })
                .fold(0, |acc, x| {
                    if let BoardSlot::UnMarked(val) = x {
                        acc + val
                    } else {
                        acc
                    }
                });

            acc + suma
        })
    }
}

fn parse_numbers(numbers: &str) -> Vec<i32> {
    numbers
        .split(',')
        .map(|s| s.parse().expect("could not parse char to u32"))
        .collect()
}

fn get_input() -> Vec<String> {
    include_str!("./inputs/day4.real")
        .trim_end()
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|s| s.to_owned())
        .collect()
}

pub fn solve_one() {
    let input = get_input();
    let numbers = parse_numbers(&input[0]);
    let mut boards: Vec<Board> = input[1..].chunks(5).map(Board::from_strings).collect();
    println!("{:?}", numbers);
    // println!("{:#?}", boards);

    for n in numbers {
        for b in &mut boards {
            if b.mark(n) {
                let sum = b.sum();
                println!("{:#?}\n{} - {}\n{}", b, sum, n, sum * n);
                return;
            }
        }
    }
}

#[derive(Debug)]
struct WinningBoard {
    idx: usize,
    sum: i32,
    num: i32,
}

pub fn solve_two() {
    let input = get_input();
    let numbers = parse_numbers(&input[0]);
    let mut boards: Vec<Board> = input[1..].chunks(5).map(Board::from_strings).collect();
    let mut winner_boards_indices: Vec<WinningBoard> = Vec::with_capacity(boards.len() / 2);
    println!("{:?}", numbers);

    for n in numbers {
        'middle: for (i, b) in &mut boards.iter_mut().enumerate() {
            for wb in &winner_boards_indices {
                if wb.idx == i {
                    continue 'middle;
                }
            }

            if b.mark(n) {
                winner_boards_indices.push(WinningBoard {
                    idx: i,
                    sum: b.sum(),
                    num: n,
                });
            }
        }
    }

    let last_winner = winner_boards_indices.last().expect("No winning boards");
    println!("{}", last_winner.sum * last_winner.num);
}
