use std::io::{self, Write};


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
    
    for (byte_index, &byte) in board.iter().enumerate() {

        // Each u8 contains 4 2-bit states, extract each of them
        for bit_offset in 0..4 {

            let state = (byte >> (bit_offset*2)) & 0b11; // Extract 2bit state
            
            match count % 3 {
               0 => print!(" {} |",to_marker(&state)),
               1 => print!(" {} |",to_marker(&state)),
               _ => print!(" {} \n",to_marker(&state)),
            }

            count += 1; // Increment counter

            // Print a line after every 3 states, but not after the last row
            if count % 3 == 0 && count < 9 {
                println!("---|---|---");
            }

            if count >= 9 {
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
    
    println!("Numpad Layout:");
    println!("  7 | 8 | 9");
    println!(" ---|---|---");
    println!("  4 | 5 | 6");
    println!(" ---|---|---");
    println!("  1 | 2 | 3");

    println!();
}

fn main() {
    let mut board = [0u8; 3]; // Each u8 can store 4 cells (2 bits per cell)
    let mut current_player = 1; // 1 for "x", 2 for "o"

    print_instructions();

    loop {
        // Print the updated board
        println!("Tic Tac Toe");
        print_board(&board);
        print!("\n");

        // Ask for user input
        print!("Player {} ({}), enter a position (1-9): ", current_player, if current_player == 1 { "x" } else { "o" });
        io::stdout().flush().expect("Failed to flush stdout"); // Flush to ensure the prompt prints before reading input
        
        let mut input = String::new();
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

        // Switch player for the next turn
        current_player = if current_player == 1 { 2 } else { 1 };
    }
}
