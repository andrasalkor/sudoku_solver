use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};

pub struct SudokuGrid {
    cells: [[u8; 9]; 9]
}

impl SudokuGrid {
    pub fn new() -> Self {
        Self {
            cells: [[0; 9]; 9]
        }
    }

    pub fn display(&self) {
        for row in &self.cells {
            let mut line = String::new();
            for &cell in row {
                let cell_str = if cell == 0 { ". ".to_string() } else { cell.to_string() + " " };
                line.push_str(&cell_str);
            }
            println!("{line}");
        }
    }

    pub fn read_from_file(file_path: &str) -> Result<Self, Error> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);

        let mut sudoku_grid = SudokuGrid::new();

        for (row, line) in reader.lines().enumerate() {
            let line = line?;
            let chars: Vec<char> = line.chars().collect();

            for (col, ch) in chars.iter().enumerate() {
                if let Some(num) = ch.to_digit(10) {
                    sudoku_grid.set_cell(row, col, num as u8);
                } else if *ch != '.' {
                    return Err(Error::new(ErrorKind::InvalidData, "Invalid character in input."));
                }
            }
        }
        Ok(sudoku_grid)
    }

    fn get_cell(&self, row: usize, col: usize) -> u8 {
        self.cells[row][col]
    }

    fn set_cell(&mut self, row: usize, col: usize, value: u8) {
        self.cells[row][col] = value;
    }

    fn find_empty_cell(&self) -> Option<(usize, usize)> {
        for row in 0..9 {
            for col in 0..9 {
                if self.get_cell(row, col) == 0 {
                    return Some((row, col));
                }
            }
        }
        None
    }

    fn is_in_col(&self, col: usize, num: u8) -> bool {
        for row in 0..9 {
            if self.get_cell(row, col) == num {
                return true;
            }
        }
        false
    }

    fn is_in_row(&self, row: usize, num: u8) -> bool {
        for col in 0..9 {
            if self.get_cell(row, col) == num {
                return true;
            }
        }
        false
    }

    fn is_in_subgrid(&self, start_row: usize, start_col: usize, num: u8) -> bool {
        for row in 0..3 {
            for col in 0..3 {
                if self.get_cell(row + start_row, col + start_col) == num {
                    return true;
                }
            }
        }
        false
    }

    fn is_valid_move(&self, row: usize, col: usize, num: u8) -> bool {
        !self.is_in_col(col, num) && !self.is_in_row(row, num) && !self.is_in_subgrid(row - row % 3, col - col % 3, num)
    }

    pub fn solve(&mut self) -> bool {
        if let Some((row, col)) = self.find_empty_cell() {
            for num in 1..=9 {
                if self.is_valid_move(row, col, num as u8) {
                    self.set_cell(row, col, num as u8);
                    if self.solve() {
                        return true;
                    }
                    self.set_cell(row, col, 0);
                }
            }
            return false;
        }
        true
    }
}
