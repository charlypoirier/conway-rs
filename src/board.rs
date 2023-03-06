use rand::thread_rng;
use rand::Rng;

#[derive(Clone, Copy, PartialEq)]
enum CellState {
    Dead,
    Alive,
}

pub struct Board {
    cells: Vec<Vec<CellState>>,
}

impl Board {
    pub fn new(cols: usize, rows: usize) -> Board {
        let mut cells = vec![vec![CellState::Dead; cols]; rows];
        let mut rng = thread_rng();
        for row in 0..rows {
            for col in 0..cols {
                if rng.gen_bool(0.1) {
                    cells[row][col] = CellState::Alive;
                }
            }
        }
        Board { cells }
    }

    pub fn update(&mut self) {
        let rows = self.cells.len();
        let cols = self.cells[0].len();
        let mut updated_state = vec![vec![CellState::Dead; cols]; rows];

        for i in 0..rows {
            for j in 0..cols {
                let mut neighbors = 0;
                for k in (i.saturating_sub(1))..=(i + 1).min(rows - 1) {
                    for l in (j.saturating_sub(1))..=(j + 1).min(cols - 1) {
                        if (k != i || l != j) && self.cells[k][l] == CellState::Alive {
                            neighbors += 1;
                        }
                    }
                }

                if self.cells[i][j] == CellState::Dead {
                    if neighbors == 3 {
                        updated_state[i][j] = CellState::Alive;
                    }
                } else if neighbors > 1 && neighbors < 4 {
                    updated_state[i][j] = CellState::Alive;
                }
            }
        }

        self.cells = updated_state;
    }

    pub fn display(&self) {
        for i in 0..self.cells.len() {
            for j in 0..self.cells[i].len() {
                match self.cells[i][j] {
                    CellState::Alive => print!("█"),
                    CellState::Dead => print!(" "),
                }
            }
        }
    }
}
