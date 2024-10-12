use std::io::{self, stdout, Write};
use crossterm::{execute, terminal::{Clear, ClearType}};
use std::time::Duration;
use std::thread::sleep;

fn is_winner(board: &[u8], player: u8) -> bool {
    // Define 8 possible win conditions
    let win_conditions = [
        [0, 1, 2], // Row  1
        [3, 4, 5], // Row  2
        [6, 7, 8], // Row  3
        [0, 3, 6], // Col  1
        [1, 4, 7], // Col  2
        [2, 5, 8], // Col  3
        [0, 4, 8], // Diag 1
        [2, 4, 6], // Diag 2
    ];

    // Check if the player occupies all positions in any win condition
    for condition in win_conditions.iter() {
        if condition.iter().all(|&index| get_state(board, index) == player) {
            return true;
        }
    }
    false
    
}

fn is_draw(board: &[u8]) -> bool {
    // board full? no winner, draw game
    (0..9).all(|i| get_state(board, i) != 0) && !is_winner(board, 1) && !is_winner(board, 2)
}

fn set_state(board: &mut [u8], index: usize, state: u8) {

    // Calculate which u8 holds the desired 2-bit state
    let byte_index = index / 4;
    let bit_offset = (index % 4) * 2;

    // Clear the existing 2-bit region and set new state
    board[byte_index] &= !(0b11 << bit_offset); // Clears 2 bits stored at byte_index  ex:bit_offset=4   0b110000 ! flips to 0b001111 clearing 2bits for position 
    board[byte_index] |=  (state & 0b11) << bit_offset; // Set the new state           ex:state=0b0010 (state 2) 0b11 grabs last 2 bits then shifts them to index 

}

fn get_state(board: &[u8], index: usize) -> u8 {

    // Calculate which u8 holds the desired 2-bit state
    let byte_index = index / 4;
    let bit_offset = (index % 4) * 2;

    // Clear the existing 2-bit region and set new state
    (board[byte_index] >> bit_offset) & 0b11 //returns this 2bit value

}

fn to_marker(state: &u8) -> String{
    match state {
        1 => "x".to_string(),
        2 => "o".to_string(),
        _ => " ".to_string(),
    }
}

fn print_board(board: &[u8]) {

    let mut count = 0;
    
    for (_byte_index, &byte) in board.iter().enumerate() {

        // Each u8 contains 4 2-bit states, extract each of them
        for bit_offset in 0..4 {

            let state = (byte >> (bit_offset*2)) & 0b11; // Extract 2bit state
            
            match count % 3 {
               0 => print!("     {} |",to_marker(&state)),
               1 => print!(" {} |",to_marker(&state)),
               _ => print!(" {} \n",to_marker(&state)),
            }

            count += 1; // Increment counter

            // Print a line after every 3 states, but not after the last row
            if count % 3 == 0 && count < 9 {
                println!("    ---|---|---");
            }

            if count >= 9 {
                println!();
                return; // Stop after 9 states
            }

        }
    }

}

fn numpad_to_index(num: usize) -> Option<usize> {
    match num {
        7 => Some(0),
        8 => Some(1),
        9 => Some(2),
        4 => Some(3),
        5 => Some(4),
        6 => Some(5),
        1 => Some(6),
        2 => Some(7),
        3 => Some(8),
        _ => None, // Invalid input
    }
}

fn print_instructions() {
    println!("Welcome to Tic Tac Toe!");
    println!();
    println!("How to play:");
    println!("You will be prompted to enter a position on the board.");
    println!("Use the corresponding numpad keys to make your move.");
    println!("Each player takes turns placing their marker ('x' or 'o').");
    println!();
    
    println!("   Numpad Layout:\n");
    println!("     7 | 8 | 9");
    println!("    ---|---|---");
    println!("     4 | 5 | 6");
    println!("    ---|---|---");
    println!("     1 | 2 | 3");

    println!();
    pause_for_enter();
}

fn clear_screen() {
    // Clear the entire terminal screen
    execute!(stdout(), Clear(ClearType::All)).expect("Failed to clear screen");
}

fn pause_for_enter() {
    let mut input = String::new();
    println!("Press [ENTER] to begin...");
    io::stdout().flush().expect("Failed to flush stdout"); // Make sure the prompt is printed
    io::stdin().read_line(&mut input).expect("Failed to read line");
}

fn minimax(board: &mut [u8], depth: i32, is_maximizing: bool) -> i32 {
    if is_winner(board, 2){
        return 1;  // AI wins
    } else if is_winner(board, 1) {
        return -1; // Player Wins
    } else if is_draw(board) {
        return 0;  // Draw
    }

    if is_maximizing {
        let mut best_score = -100;
        for i in 0..9 {
            if get_state(board, i) == 0 { // If the ccell is empty
                set_state(board, i, 2);   // AI makes the move
                let score = minimax(board, depth + 1, false);
                set_state(board, i, 0);   // Undo move
                best_score = best_score.max(score);
            }
        }
        return best_score;
    } else {
        let mut best_score = 100;
        for i in 0..9 {
            if get_state(board, i) == 0 { // If the cell is empty
                set_state(board, i, 1);   // Player makes the move
                let score = minimax(board, depth + 1, true);
                set_state(board, i, 0);   // Undo move
                best_score = best_score.min(score);
            }
        }
        return best_score;
    }

}

fn find_best_move(board: &mut [u8]) -> usize {
    let mut best_move = 0;
    let mut best_score = -100;

    for i in 0..9 {
        if get_state(board, i) == 0 { // If cell is empty
            set_state(board, i , 2);  // AI makes the move
            let score = minimax(board, 0, false);
            set_state(board, i, 0); // Undo move

            if score > best_score {
                best_score = score;
                best_move = i;
            }
        }
    }

    return best_move;
}

fn main() {
    let mut board = [0u8; 3]; // Each u8 can store 4 cells (2 bits per cell)
    let mut current_player = 1; // 1 for "x", 2 for "o"
    
    
    print_instructions();

    loop {
        if current_player == 1 { // Human Player's turn
            clear_screen();
            println!();

            // Print the updated board
            println!("    Tic Tac Toe\n\n\n");
            print_board(&board);
            print!("\n");

            // Ask for user input
            let mut input = String::new();
            print!("Player {} ({}), enter a position (1-9): ", current_player, if current_player == 1 { "x" } else { "o" });
            io::stdout().flush().expect("Failed to flush stdout"); // Flush to ensure the prompt prints before reading input

            io::stdin().read_line(&mut input).expect("Failed to read line");

            // Try to parse input as a number
            let position: usize = match input.trim().parse::<usize>().ok().and_then(numpad_to_index) {
                Some(index) => index,
                None => {
                    println!("Invalid input. Please enter a number between 1 and 9.");
                    continue;
                }
            };

            // Check if the cell is already occupied
            if get_state(&board, position) != 0 {
                println!("That position is already taken. Please try again.");
                continue;
            }

            // Set the state on the board (1 for "x", 2 for "o")
            set_state(&mut board, position, current_player);

        }else { // AI's turn

            clear_screen();
            println!("    Tic Tac Toe\n");
            println!("    Thinking.\n");
            print_board(&board);
            println!();
            sleep(Duration::from_millis(300));
            clear_screen();
            println!("    Tic Tac Toe\n");
            println!("    Thinking..\n");
            print_board(&board);
            println!();
            sleep(Duration::from_millis(200));
            clear_screen();
            println!("    Tic Tac Toe\n");
            println!("    Thinking...\n");
            print_board(&board);
            println!();
            sleep(Duration::from_millis(100));

            // Find the best move for the AI 
            let best_move = find_best_move(&mut board);
            set_state(&mut board, best_move, 2); // Set AI's move on the board

        }

        // Check win condition
        if is_winner(&board, current_player) {
            clear_screen();
            println!("\n    Player {} ({}) wins!\n", current_player, if current_player == 1 { "x" } else { "AI" });
            print_board(&board);
            println!();
            break; // Exit game loop when player wins
        }

        // Check draw condition
        if is_draw(&board) {
            clear_screen();
            println!("\n    It's a Draw!\n");
            print_board(&board);
            println!();
            break; // Exit game loop when player wins
        }

        // Switch player for the next turn
        current_player = if current_player == 1 { 2 } else { 1 };
        
    }
}
