use std::fmt;
use std::{thread, time};

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
}

fn main() {
    let mut board = Board::new(30, 60);
    board.make_alive(vec![
        (1, 5),
        (1, 6),
        (2, 5),
        (2, 6),

        (12, 4),
        (13, 3),
        (14, 3),
        (16, 4),
        (17, 5),
        (17, 6),
        (17, 7),
        (18, 6),
        (16, 8),
        (14, 9),
        (13, 9),
        (12, 8),
        (11, 7),
        (11, 6),
        (11, 5),
        (15, 6),

        (21, 3),
        (21, 4),
        (21, 5),
        (22, 3),
        (22, 4),
        (22, 5),
        (23, 2),
        (23, 6),

        (25, 1),
        (25, 2),
        (25, 6),
        (25, 7),

        (35, 3),
        (35, 4),
        (36, 3),
        (36, 4),
    ]);

    print!("[?25l");      // ANSI sequence for hiding the cursor
    loop {
        print!("{}", board);
        print!("[{}A[{}D", board.rows, board.cols); // Reset the cursor position

        board.iter();
        thread::sleep(time::Duration::from_millis(100));
    }
}
