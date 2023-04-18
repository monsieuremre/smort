// This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
// This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
// You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
//
// SMORT - Sudoku Meets Optimized Recursive Traversal <https://github.com/monsieuremre/smort>

use std::io::{self, Write};
use std::{thread, time::Duration};

// Possible states
enum GameState {
    Solved,
    NotSolvable,
    WaitingEntry,
    Quit,
}

// Main function
fn main() {
    let mut state: GameState = GameState::WaitingEntry;
    let mut constant_map: [bool; 81] = [false; 81]; // Tells if a digit was given by the user or did we put it there when solving
    let mut board: [i32; 81] = [0; 81]; // The board represented as an array from left to right and top to bottom
                                        // Using a loop for the output creates continuity
    loop {
        print!("{esc}c", esc = 27 as char); // Clear the terminal every time to look continuous
        println!("SMORT - Sudoku Meets Optimized Recursive Traversal");
        print_board(board, constant_map);
        match state {
            GameState::WaitingEntry => wait_entry(&mut board, &mut constant_map, &mut state),
            GameState::Solved => is_solved(&mut state),
            GameState::NotSolvable => no_solution(&mut state),
            GameState::Quit => break,
        }
    }
}

// Wait for entry in a loop
fn wait_entry(board: &mut [i32; 81], constant_map: &mut [bool; 81], state: &mut GameState) {
    print_options();
    io::stdout().flush().unwrap();
    // Take input
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");
    // Check if input is a valid character
    if let Ok(input) = input.trim().parse::<char>() {
        match input {
            'E' => add_number(board, constant_map),
            'R' => *board = [0; 81],
            'D' => delete_number(board, constant_map),
            'S' => solve_board(board, *constant_map, state, false),
            'V' => solve_board(board, *constant_map, state, true),
            'U' => unsolve_board(board, *constant_map),
            'Q' => *state = GameState::Quit,
            _ => println!("Invalid Input!"),
        }
    } else {
        println!("Invalid Input!");
    }
}

// We print and wait for 3 seconds for readability
fn is_solved(state: &mut GameState) {
    println!("Entry Solved!");
    *state = GameState::WaitingEntry;
    thread::sleep(Duration::from_secs(3));
}

// Same, we print and wait for 3 seconds for readability
fn no_solution(state: &mut GameState) {
    println!("There is no valid solution to this sudoku puzzle!");
    *state = GameState::WaitingEntry;
    thread::sleep(Duration::from_secs(3));
}

// Printing board. Numbers determined by the user are printed with color
fn print_board(board: [i32; 81], constant_map: [bool; 81]) {
    for i in 0..81 {
        if board[i] == 0 {
            print!("[ ]"); // Empty box
        } else if constant_map[i] {
            // This is not part of the solution, so colored
            print!("[\x1b[93m{}\x1b[0m]", board[i]);
        } else {
            print!("[{}]", board[i]); // Regular box
        }
        if (i + 1) % 9 == 0 {
            print!("\n");
        } else {
            print!(" ");
        }
    }
    io::stdout().flush().unwrap();
}

// Recursively solve board and change the game status accordingly
fn solve_board(
    board: &mut [i32; 81],
    constant_map: [bool; 81],
    state: &mut GameState,
    visualize: bool,
) {
    if recursive_solve(board, constant_map, visualize) {
        *state = GameState::Solved;
    } else {
        *state = GameState::NotSolvable;
    }
}

// The actual recursive function for solving
fn recursive_solve(board: &mut [i32; 81], constant_map: [bool; 81], visualize: bool) -> bool {
    let next = find_empty_box(*board);
    match next {
        None => return true,
        Some((row, col)) => {
            for i in 1..10 {
                // Visualize if visualization mode is selected
                if visualize {
                    board[(row * 9 + col) as usize] = i;
                    print!("{esc}c", esc = 27 as char);
                    println!("SMORT - Sudoku Meets Optimized Recursive Traversal");
                    print_board(*board, constant_map);
                    println!("Solving Slowly and Printing Each Step for Visualization");
                    thread::sleep(Duration::from_millis(100));
                    board[(row * 9 + col) as usize] = 0;
                }
                // Recursion occurs here
                if is_valid(*board, i, row, col) {
                    board[(row * 9 + col) as usize] = i;
                    if recursive_solve(board, constant_map, visualize) {
                        return true;
                    }
                    board[(row * 9 + col) as usize] = 0;
                }
            }
        }
    }
    return false;
}

// Remove the solution from board
fn unsolve_board(board: &mut [i32; 81], constant_map: [bool; 81]) {
    for i in 0..81 {
        if !constant_map[i] {
            board[i] = 0;
        }
    }
}

// Check if suggester number can be in the suggested position
fn is_valid(board: [i32; 81], val: i32, row: i32, col: i32) -> bool {
    //Check the row
    for i in 0..9 {
        if (board[(row * 9 + i) as usize] == val) && col != i {
            return false;
        }
    }
    // Check the column
    for i in 0..9 {
        if (board[(i * 9 + col) as usize] == val) && row != i {
            return false;
        }
    }
    // Check the 3x3 box
    let square_row: i32 = row / 3;
    let square_col: i32 = col / 3;
    for i in 0..3 {
        for j in 0..3 {
            if board[((square_row * 3 + i) * 9 + square_col * 3 + j) as usize] == val
                && row != i
                && col != j
            {
                return false;
            }
        }
    }
    return true;
}

// Find the next empty box
fn find_empty_box(board: [i32; 81]) -> Option<(i32, i32)> {
    for i in 0..9 {
        for j in 0..9 {
            if board[(i * 9 + j) as usize] == 0 {
                return Some((i, j));
            }
        }
    }
    return None;
}

// Add a number to the board by user, before the solution
fn add_number(board: &mut [i32; 81], constant_map: &mut [bool; 81]) {
    let row: i32;
    let col: i32;
    let val: i32;
    println!("Adding new number to the board. Enter details.");
    // Take details from user using a loop until a valid input is provided
    // Taking row information
    print!("Row of the number: ");
    io::stdout().flush().unwrap();
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");
        if let Ok(input) = input.trim().parse::<i32>() {
            match input {
                1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 => {
                    row = input;
                    break;
                }
                _ => println!("Invalid Input!"),
            }
        } else {
            println!("Invalid Input!");
        }
    }
    // Taking column information
    print!("Column of the number: ");
    io::stdout().flush().unwrap();
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");
        if let Ok(input) = input.trim().parse::<i32>() {
            match input {
                1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 => {
                    col = input;
                    break;
                }
                _ => println!("Invalid Input!"),
            }
        } else {
            println!("Invalid Input!");
        }
    }
    // Taking the value
    print!("Value of the number: ");
    io::stdout().flush().unwrap();
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");
        if let Ok(input) = input.trim().parse::<i32>() {
            match input {
                1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 => {
                    val = input;
                    break;
                }
                _ => println!("Invalid Input!"),
            }
        } else {
            println!("Invalid Input!");
        }
    }
    // Add the user value to the board and modify the map of constant digits accordingly
    board[((row - 1) * 9 + col - 1) as usize] = val;
    constant_map[((row - 1) * 9 + col - 1) as usize] = true;
}

// Delete number from the board, again before the solution
fn delete_number(board: &mut [i32; 81], constant_map: &mut [bool; 81]) {
    let row: i32;
    let col: i32;
    println!("Deleting a number to the board. Enter details.");
    // Take details from user using a loop until a valid input is provided
    // Taking row information
    print!("Row of the number: ");
    io::stdout().flush().unwrap();
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");
        if let Ok(input) = input.trim().parse::<i32>() {
            match input {
                1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 => {
                    row = input;
                    break;
                }
                _ => println!("Invalid Input!"),
            }
        } else {
            println!("Invalid Input!");
        }
    }
    // Taking column information
    print!("Column of the number: ");
    io::stdout().flush().unwrap();
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");
        if let Ok(input) = input.trim().parse::<i32>() {
            match input {
                1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 => {
                    col = input;
                    break;
                }
                _ => println!("Invalid Input!"),
            }
        } else {
            println!("Invalid Input!");
        }
    }
    // Add the user value to the board and modify the map of constant digits accordingly
    board[((row - 1) * 9 + col - 1) as usize] = 0;
    constant_map[((row - 1) * 9 + col - 1) as usize] = false;
}

// Print a list of options user can select
fn print_options() {
    println!("List of Commands:");
    println!("[E]nter value to board");
    println!("[D]elete value from board");
    println!("[R]eset board values");
    println!("[S]olve the board");
    println!("[V]isualize and solve the board");
    println!("[U]nsolve the board");
    println!("[Q]uit");
    print!("Enter Command: ");
}
