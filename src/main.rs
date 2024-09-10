mod processor;
mod tests;
mod opcode_info;
mod snake;
use crate::processor::CPU;
use std::io;

fn main() {

    print!("\n");
    print!(" ▄▄▄▄▄▄▄▄▄▄▄  ▄▄▄▄▄▄▄▄▄▄▄   ▄▄▄▄▄▄▄▄▄   ▄▄▄▄▄▄▄▄▄▄▄ 
▐░░░░░░░░░░░▌▐░░░░░░░░░░░▌ ▐░░░░░░░░░▌ ▐░░░░░░░░░░░▌
▐░█▀▀▀▀▀▀▀▀▀ ▐░█▀▀▀▀▀▀▀▀▀ ▐░█░█▀▀▀▀▀█░▌ ▀▀▀▀▀▀▀▀▀█░▌
▐░▌          ▐░█▄▄▄▄▄▄▄▄▄ ▐░▌▐░▌    ▐░▌          ▐░▌
▐░█▄▄▄▄▄▄▄▄▄ ▐░░░░░░░░░░░▌▐░▌ ▐░▌   ▐░▌          ▐░▌
▐░░░░░░░░░░░▌ ▀▀▀▀▀▀▀▀▀█░▌▐░▌  ▐░▌  ▐░▌ ▄▄▄▄▄▄▄▄▄█░▌
▐░█▀▀▀▀▀▀▀█░▌          ▐░▌▐░▌   ▐░▌ ▐░▌▐░░░░░░░░░░░▌
▐░▌       ▐░▌          ▐░▌▐░▌    ▐░▌▐░▌▐░█▀▀▀▀▀▀▀▀▀ 
▐░█▄▄▄▄▄▄▄█░▌ ▄▄▄▄▄▄▄▄▄█░▌▐░█▄▄▄▄▄█░█░▌▐░█▄▄▄▄▄▄▄▄▄ 
▐░░░░░░░░░░░▌▐░░░░░░░░░░░▌ ▐░░░░░░░░░▌ ▐░░░░░░░░░░░▌
 ▀▀▀▀▀▀▀▀▀▀▀  ▀▀▀▀▀▀▀▀▀▀▀   ▀▀▀▀▀▀▀▀▀   ▀▀▀▀▀▀▀▀▀▀▀");
    println!("\n \n Welcome to the 6502! \n 1. To write code type and enter write \n 2. To run snake type and enter snake \n");
    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    choice = choice.trim().to_lowercase();

    if choice == "snake" {
        snake::run_snake();

    } else if choice != "write" {
        std::process::exit(0);  
    } 
}