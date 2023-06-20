extern crate ndarray;
extern crate rayon;
extern crate time;

use ndarray::Array2;
use rayon::prelude::*;
use std::sync::mpsc;
use time::Instant;

struct Solver {
    start_time: Instant,
    seed: String,
    board: Array2<i32>,
}

impl Solver {
    fn new(seed: String) -> Solver {
        let start_time = Instant::now();
        let board = Solver::from_iter(seed.clone());
        Solver { start_time, seed, board }
    }

    fn from_iter(seed: String) -> Array2<i32> {
        let mut arr = Array2::from_elem((9, 9), 0);
        for (i, num) in seed.chars().enumerate() {
            arr[[i / 9, i % 9]] = num.to_digit(10).unwrap() as i32;
        }
        arr
    }

    fn subgrid(n: usize) -> usize {
        match n {
            0..=2 => 0,
            3..=5 => 3,
            6..=8 => 6,
            _ => panic!("Invalid index for subgrid"),
        }
    }

    fn available(&self, row: usize, col: usize) -> Vec<i32> {
        let x = Solver::subgrid(row);
        let y = Solver::subgrid(col);
        
        let mut used_numbers = Vec::new();
        used_numbers.extend(self.board.row(row).iter().copied());
        used_numbers.extend(self.board.column(col).iter().copied());
        used_numbers.extend(self.board.slice(s![x..x+3, y..y+3]).iter().copied());
        used_numbers.sort_unstable();
        used_numbers.dedup();

        (0..10).filter(|n| !used_numbers.contains(&(*n as i32))).collect()
    }

    fn solve(&mut self, tx: mpsc::Sender<()>, rx: mpsc::Receiver<()>) {
        for row in 0..9 {
            for col in 0..9 {
                if self.board[[row, col]] == 0 {
                    for val in self.available(row, col) {
                        self.board[[row, col]] = val;
                        let board_clone = self.board.clone();
                        let (tx2, rx2) = mpsc::channel();
                        let tx_clone = tx.clone();
                        rayon::spawn(move || {
                            self.solve(tx_clone, rx2);
                        });
                        if rx.try_recv().is_ok() {
                            return;
                        }
                        self.board[[row, col]] = 0;
                    }
                    return;
                }
            }
        }
        println!("Processing time: {}", self.start_time.elapsed().as_secs_f64());
        println!("{:?}", self.board);
        tx.send(()).unwrap();
    }
}

fn main() {
    let seed = "800000000003600000070090200050007000000045700000100030001000068008500010090000400".to_string();
    let mut sudoku = Solver::new(seed);
    let (tx, rx) = mpsc::channel();
    sudoku.solve(tx, rx);
}
