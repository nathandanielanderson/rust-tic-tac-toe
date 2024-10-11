use std::io;

fn main() {

    //p1 represented by u16 0b000000000[<-Position | Health->]0000000
    //                      0b012345678                       0 - 100
    //p2 same as p1
    //board represent by u16 0b000000000[<-Position | Turn->]0000000
    //                       0b012345678                          21      01 << = 10 .... 10 >> = 01
    // Create a mutable String to store user input
    let mut input = String::new();
    let mut p1: u8 = 0;
    println!("Player 1's intial state: {:?}", p1);
    let board: [[bool; 3]; 3] = [[false; 3]; 3];
    println!("Board's intial state: {:?}", board);



}

fn set_state(board: &mut [u8], index: usize, state: u8) {
    
}