use std::{array, collections::HashSet, io::empty, time::Duration};
use colored::*;
use rand::{seq::SliceRandom, Rng};
use std::thread::sleep;
use std::time::Instant;

struct Sudoku {
    grid: [[char; 9]; 9],
    steps: Vec<Step>,
}

struct Step {
    x: usize,
    y: usize,
    char: char,
    original: bool,
}

impl Step {
    fn new(x: usize, y: usize, char: char, original: bool) -> Self {
        Self {
            x,
            y,
            char,
            original,
        }
    }
}

impl Sudoku {
    fn new() -> Self {
        Self {
            grid: [['.'; 9];9],
            steps: Vec::new(),
        }
    }

    fn populate(&mut self, x: usize, y: usize, value: char ) {
        self.grid[x][y] = value;
    }

    fn populate_grid(&mut self, x: usize, y: usize, arr: [char; 9] ) {
        if x <= 2 && y <= 2 {
            let mut array_index = 0;
            for new_y in 0..3 {
                for new_x in 0..3 {
                    self.grid[y * 3 + new_y][x * 3 + new_x] = arr[array_index];
                    array_index += 1;
                }
            }
        }
    }

    fn remove_entry(&mut self, x: usize, y: usize ) {
        self.grid[x as usize][y as usize] = '.';
    }

    fn print(&self) {
        let mut invalid_characters = Vec::new();
        for x in 0..9 {
            for y in 0..9  {
                if !Self::check_pos_for_valid(&self, x, y) {
                    invalid_characters.push((x, y));
                    println!("Invalid at: {}, {}", x, y)
                }
            }
        }
        for x in 0..9 {
            for y in 0..9  {
                let mut invalid = false;
                for (invalid_x, invalid_y) in invalid_characters.iter() {
                    if *invalid_x == x && *invalid_y == y {
                        invalid = true;
                        break;
                    }
                }
                let char = self.grid[x][y].to_string();
                if y == 3 || y == 6 {
                    print!("|")
                }
                print!("{}", if invalid {char.red()} else { char.white() })
            }
            if x == 2 || x == 5 {
                println!("");
                println!("-----------");
            }
            else {
                println!("");
            }
        }
    }

    fn check_sudoku_valid(&self) -> bool{
        for x in 0..9 {
            for y in 0..9  {
                if !Self::check_pos_for_valid(&self, x, y) {
                    return false;
                }
            }
        }
        true
    }

    fn solve(&mut self, cycles: usize) {
        let mut empty_locations: Vec<(usize, usize)> = Vec::new();
        let mut original_cells = [[false; 9]; 9];

        // Collect empty cells and mark original cells
        for x in 0..9 {
            for y in 0..9 {
                if self.grid[x][y] != '.' {
                    original_cells[x][y] = true;
                } else {
                    empty_locations.push((x, y));
                }
            }
        }

        if empty_locations.is_empty() {
            println!("Puzzle is already solved!");
            return;
        }

        // Initialize the steps stack
        let mut steps: Vec<Step> = Vec::new();

        // Start from the first empty cell
        let mut current_index = 0;
        let mut current_step = Step::new(
            empty_locations[current_index].0,
            empty_locations[current_index].1,
            '1',
            false,
        );

        for i in 0..cycles {
            let (pos_x, pos_y, pos_char) = (current_step.x, current_step.y, current_step.char);

            // Try to place the current character
            self.populate(pos_x, pos_y, pos_char);

            if !self.check_pos_for_valid(pos_x, pos_y) {
                // Invalid placement, remove and try next number
                self.remove_entry(pos_x, pos_y);
                let new_value = pos_char.to_digit(10).unwrap() + 1;

                if new_value <= 9 {
                    // Try the next number for this cell
                    current_step.char = char::from_digit(new_value, 10).unwrap();
                } else {
                    // Need to backtrack
                    self.grid[pos_x][pos_y] = '.'; // Reset the cell

                    loop {
                        if steps.is_empty() {
                            self.print();
                            println!("No solution exists!");
                            return;
                        }

                        // Backtrack to the previous cell
                        current_step = steps.pop().unwrap();
                        current_index -= 1;
                        let new_value = current_step.char.to_digit(10).unwrap() + 1;

                        if new_value <= 9 {
                            // Try the next number for this cell
                            current_step.char = char::from_digit(new_value, 10).unwrap();
                            break; // Exit loop to retry with new value
                        } else {
                            // Reset and continue backtracking
                            self.grid[current_step.x][current_step.y] = '.';
                            // Continue backtracking
                        }
                    }
                }
            } else {
                // Valid placement, move to the next cell
                steps.push(current_step);
                current_index += 1;

                if current_index == empty_locations.len() {
                    // Puzzle solved
                    self.print();
                    println!("Cycle: {}", i);
                    println!("Sudoku solved!");
                    return;
                }

                // Prepare the next step
                let (next_x, next_y) = empty_locations[current_index];
                current_step = Step::new(next_x, next_y, '1', false);
            }
        }

        // If the loop completes without returning, the puzzle was not solved
        self.print();
        println!("Reached cycle limit without solving the puzzle.");
    }

    fn check_pos_for_valid(&self, x: usize, y: usize) -> bool{
        let char = self.grid[x][y];

        if char == '.' {
            return true;
        }

        for new_x in 0..9 {
            if new_x != x && self.grid[new_x][y] == char {
                return false;
            }
        }

        for new_y in 0..9 {
            if new_y != y && self.grid[x][new_y] == char {
                return false;
            }
        }

        let grid_x = (x / 3) * 3;
        let grid_y = (y / 3) * 3;

        for xi in grid_x..grid_x + 3 {
            for yj in grid_y..grid_y + 3 {
                if xi == x && yj == y {
                    continue;
                }
                if self.grid[xi][yj] == char {
                    return false;
                }
            }
        }
        true
    }

    fn fill(&mut self, sudoku_board: [[char; 9 ]; 9]) {
        self.grid = sudoku_board.clone()
    }
}

fn main() {
    let mut sudoku = Sudoku::new();

    let sudoku_grid_easy = [
        ['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        ['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        ['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        ['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        ['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        ['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        ['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        ['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        ['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];
    let sudoku_grid_medium = [
        ['8', '.', '.', '.', '.', '.', '.', '.', '.'],
        ['.', '.', '3', '6', '.', '.', '.', '.', '.'],
        ['.', '7', '.', '.', '9', '.', '2', '.', '.'],
        ['.', '5', '.', '.', '.', '7', '.', '.', '.'],
        ['.', '.', '.', '.', '4', '5', '7', '.', '.'],
        ['.', '.', '.', '1', '.', '.', '.', '3', '.'],
        ['.', '.', '1', '.', '.', '.', '.', '6', '8'],
        ['.', '.', '8', '5', '.', '.', '.', '1', '.'],
        ['.', '9', '.', '.', '.', '.', '4', '.', '.'],
    ];
    let sudoku_grid_hard = [
        ['.', '.', '5', '3', '.', '.', '.', '.', '.'],
        ['8', '.', '.', '.', '.', '.', '.', '2', '.'],
        ['.', '7', '.', '.', '1', '.', '5', '.', '.'],
        ['4', '.', '.', '.', '.', '5', '3', '.', '.'],
        ['.', '1', '.', '.', '7', '.', '.', '.', '6'],
        ['.', '.', '3', '2', '.', '.', '.', '8', '.'],
        ['.', '6', '.', '5', '.', '.', '.', '.', '9'],
        ['.', '.', '4', '.', '.', '.', '.', '3', '.'],
        ['.', '.', '.', '.', '.', '9', '7', '.', '.'],
    ];
    let sudoku_grid_very_hard = [
        ['.', '.', '4', '.', '.', '.', '6', '3', '.'],
        ['.', '.', '.', '.', '.', '.', '.', '.', '.'],
        ['.', '.', '.', '.', '.', '.', '8', '.', '2'],
        ['.', '.', '.', '5', '.', '.', '.', '.', '9'],
        ['.', '.', '.', '.', '7', '.', '.', '.', '.'],
        ['9', '.', '.', '.', '.', '8', '.', '.', '.'],
        ['2', '.', '3', '.', '.', '.', '.', '.', '.'],
        ['.', '.', '.', '.', '.', '.', '.', '.', '.'],
        ['.', '6', '.', '.', '.', '.', '.', '9', '.'],
    ];

    println!("Easy sudoku!");
    sudoku.fill(sudoku_grid_easy);
    sudoku.print();
    sleep(Duration::from_secs(5));
    println!("");
    println!("Start!");
    let start_time = Instant::now();
    sudoku.solve(1_000_000_000);
    let end_time = Instant::now();
    println!("Done! Took {:?}", end_time.duration_since(start_time));
    println!("");

    println!("Medium sudoku!");
    sudoku.fill(sudoku_grid_medium);
    sudoku.print();
    sleep(Duration::from_secs(5));
    println!("");
    println!("Start!");
    let start_time = Instant::now();
    sudoku.solve(1_000_000_000);
    let end_time = Instant::now();
    println!("Done! Took {:?}", end_time.duration_since(start_time));
    println!("");

    println!("Hard sudoku!");
    sudoku.fill(sudoku_grid_hard);
    sudoku.print();
    sleep(Duration::from_secs(5));
    println!("");
    println!("Start!");
    let start_time = Instant::now();
    sudoku.solve(1_000_000_000);
    let end_time = Instant::now();
    println!("Done! Took {:?}", end_time.duration_since(start_time));
    println!("");

    println!("Very hard sudoku!");
    sudoku.fill(sudoku_grid_very_hard);
    sudoku.print();
    sleep(Duration::from_secs(5));
    println!("");
    println!("Start!");
    let start_time = Instant::now();
    sudoku.solve(1_000_000_000);
    let end_time = Instant::now();
    println!("Done! Took {:?}", end_time.duration_since(start_time));
    println!("");
}
