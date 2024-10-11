use std::io;


fn set_state(board: &mut [u8], index: usize, state: u8) {
    // Calculate which u8 holds the desired 2-bit state
    let byte_index = index / 4;
    let bit_offset = (index % 4) * 2;

    // Clear the existing 2-bit region and set new state
    board[byte_index] &= !(0b11 << bit_offset); // Clears 2 bits stored at byte_index  ex:bit_offset=4   0b110000 ! flips to 0b001111 clearing 2bits for position 
    board[byte_index] |=  (state & 0b11) << bit_offset; // Set the new state           ex:state=0b0010 (state 2) 0b11 grabs last 2 bits then shifts them to index 

}


fn main() {

    let mut input = String::new();

    // A 3x3 board needs 9 2bit cells, 9 * 2 = 18, this requires 3 u8 values.
    let mut board = [0u8; 3]; // Each u8 can store 4 cells (2bits per cell)

    println!("Before: {:?}", board);

    set_state(&mut board, 0, 2);
    set_state(&mut board, 1, 1);
    set_state(&mut board, 2, 0);

    println!("After: {:?}", board);
}
