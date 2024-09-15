mod processor;
mod tests;
mod opcode_info;
mod snake;
use crate::processor::CPU;
use std::io;

fn main() {

    // Introduction logo
    print!("\n ---------------------------------------------------------------------------------------------------------\n");
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

    if choice == "snake" { // Run snake
        snake::run_snake();

    } else if choice != "write" { // Run a user script
        std::process::exit(0);  
    } 

    // If user has chosen to write their own code, this part of the program executes instead
    print!("\n ---------------------------------------------------------------------------------------------------------");

    println!("\n Write your assembly code in pairs of hex digits, with spaces between every two digits (or a byte).
    \n It is not case sensitive. Example: A9 10. Remember that addresses are written in little-endian style.
    \n To execute your code, press enter. After the program runs, it will print out all relevant information. \n");

    // Assembly code by the user
    let mut instructions = String::new();

    io::stdin()
        .read_line(&mut instructions)
        .expect("\n Invalid input, should be string");

    if instructions.len() == 0 {
        println!("\n Invalid input. Please restart and try again.");
        std::process::exit(0);
    }

    // Split the instructions into an array, where each element is a pair of hex digits
    let split_result = instructions.split(" ");
    let arr = split_result.collect::<Vec<&str>>();
    let mut program_vec: Vec<u8> = Vec::new();
    let mut double_hex_pattern = regex::Regex::new("[0-9a-fA-F]{2}");

    for element in arr {

        // Check if string slice is a valid hex pair via regex
        let match_state = double_hex_pattern.as_mut().unwrap().is_match(element);
        if match_state {

            let mut hex_result = 0;
            let mut power: u8 = 1;

            // Convert each array element into an actual, usable decimal value
            for digit in element.chars() {
                
                let value: u8 = match digit {
                    '0' => 0,
                    '1' => 1,
                    '2' => 2,
                    '3' => 3,
                    '4' => 4,
                    '5' => 5,
                    '6' => 6,
                    '7' => 7,
                    '8' => 8,
                    '9' => 9,
                    'a' => 10,
                    'b' => 11,
                    'c' => 12,
                    'd' => 13,
                    'e' => 14,
                    'f' => 15,
                    _ => 16,
                };

                // The last element of the array ends with \r\n, so this case is for the code reaches that part
                if value == 16 {
                    break
                }

                hex_result += value * u8::pow(16, power as u32);
                if power != 0 {
                    power -= 1;
                } 
            }
            
            program_vec.push(hex_result);
        }

        else {
            println!("\n Invalid input. Please restart and try again.");
            std::process::exit(0);
        }
    }

    // Now that we have an actual usable vector, load it into the program and execute it!
    let mut cpu = CPU::new();
    cpu.load_and_execute(program_vec);
    let info = cpu.info;
    println!("\n Register Accumulator: {} \n Register X: {} \n Register Y: {} \n Stack Pointer: {} \n Status Flags: {} \n Program Counter: {} \n", info[0], info[1], info[2], info[3], info[4], info[5]);

}

