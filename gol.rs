use std::fs;
use std::process;
use std::env;
use std::{thread, time};

use std::fmt;

#[derive(Clone, Copy)]
enum Cell {
    Dead,
    Alive
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Cell::Alive => "#",
            Cell::Dead  => "."
        })
    }
}

struct Board {
    rows: usize,
    cols: usize,
    cells: Vec<Vec<Cell>>
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.cells {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        write!(f, "")
    }
}

fn snap(i: isize, limit: usize) -> usize {
    if i == -1 {
        limit - 1
    } else if i == limit as isize {
        0
    } else {
        i as usize
    }
}

impl Board {
    fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            cells: vec![vec![Cell::Dead; cols]; rows]
        }
    }


    fn get(&self, x: isize, y: isize) -> Cell {
        let x = snap(x, self.cols);
        let y = snap(y, self.rows);
        self.cells[y][x]
    }

    fn new_state(&self, x: usize, y: usize) -> Cell {
        let mut live = 0;

        for dy in 0..3 {
            for dx in 0..3 {
                if dx == 1 && dy == 1 {
                    continue;
                }

                let x = x as isize + dx - 1;
                let y = y as isize + dy - 1;

                match self.get(x, y) {
                    Cell::Alive => live += 1,
                    _ => {}
                }
            }
        }

        match live {
            3 => Cell::Alive,
            2 => self.get(x as isize, y as isize),
            _ => Cell::Dead
        }
    }

    fn iter(&mut self) {
        let mut new_world = vec![vec![Cell::Dead; self.cols]; self.rows];

        for row in 0..self.rows {
            for col in 0..self.cols {
                new_world[row][col] = self.new_state(col, row);
            }
        }

        self.cells = new_world;
    }

    fn make_alive(&mut self, cells: Vec<(usize, usize)>) {
        for (col, row) in cells {
            self.cells[row][col] = Cell::Alive;
        }
    }

    fn from_image(path: &str) -> Self {
        let image = match fs::read_to_string(path) {
            Ok(source) => source,
            Err(e) => {
                eprintln!("Error: failed to read image `{}`: {}", path, e);
                process::exit(1);
            }
        };

        let lines: Vec<String> = image
            .lines()
            .map(|l| l.to_string())
            .collect();

        let rows = lines.len();
        let cols = lines[0].len();

        let mut board = Board::new(rows, cols);
        let mut alive: Vec<(usize, usize)> = vec![];

        for (row, line) in lines.iter().enumerate() {
            for (col, cell) in line.chars().enumerate() {
                match cell {
                    '#' => alive.push((col, row)),
                    '.' => {},
                    _   => {
                        eprintln!("{}:{}:{} Invalid character '{}'", path, row + 1, col + 1, cell);
                        process::exit(1);
                    }
                }
            }
        }

        board.make_alive(alive);
        board
    }
}

fn main() {
    let arguments: Vec<String> = env::args().collect();

    if arguments.len() != 2 {
        eprintln!("Usage: gol-rs [IMAGE]");
        process::exit(1);
    }

    let path = &arguments[1];
    let mut board = Board::from_image(&path);

    loop {
        print!("{}\x1b[{}A\x1b[{}D", board, board.rows, board.cols); // Reset the cursor position
        board.iter();
        thread::sleep(time::Duration::from_millis(100));
    }
}
