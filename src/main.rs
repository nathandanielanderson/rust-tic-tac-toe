use std::io;


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
        _ => "_".to_string(),
    }
}

fn print_board(board: &[u8]) {

    let mut count = 0;
    
    for (byte_index, &byte) in board.iter().enumerate() {

        // Each u8 contains 4 2-bit states, extract each of them
        for bit_offset in (0..4) { //rev() reverses loop, so most significant 2bits are extracted first

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

fn main() {

    //let mut input = String::new();

    // A 3x3 board needs 9 2bit cells, 9 * 2 = 18, this requires 3 u8 values.
    let mut board = [0u8; 3]; // Each u8 can store 4 cells (2bits per cell)

    // Retrieve BEFORE states
    println!("Initial States");
    print_board(&board);
    
    set_state(&mut board, 0, 1);
    set_state(&mut board, 1, 2);
    set_state(&mut board, 2, 1);
    set_state(&mut board, 3, 2);
    set_state(&mut board, 4, 1);
    set_state(&mut board, 5, 2);
    set_state(&mut board, 6, 1);
    set_state(&mut board, 7, 2);
    set_state(&mut board, 8, 1);

    println!("Mutated States");
    print_board(&board);
    print!("\n");
    
}
