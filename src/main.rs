use std::env;
use sudoku::SudokuGrid;

mod sudoku;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let mut sudoku_grid: SudokuGrid = SudokuGrid::read_from_file(file_path).expect("Failed to read file");

    println!("<<< SUDOKU LOADED >>>");
    sudoku_grid.display();
    println!();
    if sudoku_grid.solve() {
        println!("<<< SUDOKU SOLVED >>>");
        sudoku_grid.display();
    } else {
        println!("I can't solve this sudoku.");
    }

}
