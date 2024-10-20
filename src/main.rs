mod processor;
mod tests;
mod opcode_info;
mod snake;
use crate::processor::CPU;
use std::{env, fs};
use std::io::{self, stdout, BufReader, BufRead, Write};
use std::path::PathBuf;

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
    println!("\n\nWelcome to the 6502! \n1. To run your assembly script type and enter script \n2. To run snake type and enter snake \n");

    // Get user input
    print!("-> ");
    stdout().flush().unwrap();
    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    choice = choice.trim().to_lowercase();

    if choice == "snake" { // Run snake
        snake::run_snake();
        std::process::exit(0);  

    } else if choice != "script" { // Run user script in a text file
        std::process::exit(0);  
    } 

    loop {
        // If user has chosen to run their script in a text file, this part of the program executes instead
        print!("\n ---------------------------------------------------------------------------------------------------------\n");

        println!("\nEnter the text file name of the 6502 script you want to run (with the extension) and press enter. For example: script.txt. 
You can just use the built-in script.txt file if you want. Your code must be in pairs of hex digits, with spaces between every two digits (or a byte).
It is not case sensitive. Remember that addresses are written in little-endian style. The code executes starting at memory 0x600. 
After each instruction (not values), relevant processor information will be printed. Alternatively, enter q to quit. \n");

        print!("-> ");
        stdout().flush().unwrap();

        // See what user entered
        let mut user_input = String::new();
        
        io::stdin()
            .read_line(&mut user_input)
            .expect("\nInvalid input, input expected\n");

        // User wants to quit, terminate the program
        if user_input.trim() == "q" {
            std::process::exit(0);
        }

        let current_directory = env::current_dir()
            .expect("\nFailed to get current directory");

        // Remove any newlines
        let filename = user_input.trim();

        if !filename.ends_with(".txt") {
            println!("\nPlease provide a valid .txt file");
            continue;
        }

        // Get full path of text file
        let filepath = current_directory.join(filename);

        if !filepath.exists() {
            println!("\nFile not found! Make sure file is outside of src directory!");
            continue;
        }

        let mut instructions: Vec<String> = Vec::new();

        // Get all of our instructions from reading the text file that the user entered
        // We should get a vector of hex digits
        match read_script(filepath) {
            Ok(result) => { 
                instructions = result;

            }

            Err(e) => {
                eprintln!("Failed to read script: {}", e);
                std::process::exit(1);
            }
        }
        
        // Our program works with uu8 elements, so now we have to convert our hex strings into u8 
        let program_vec: Vec<u8> = string_to_u8_hex(instructions);
        
        // Now that we have an actual usable vector, load it into the program and execute it!
        let mut cpu = CPU::new();
        cpu.print_mode = true; // We want to print the info after each opcode
        cpu.load_and_execute(program_vec);

        // Ask user if they want to run another script
        println!("\nRun again with another script? y/n \n");

        let mut answer = String::new();
        io::stdin()
            .read_line(&mut answer)
            .expect("\nInvalid input. Should be string. \n");

        answer = answer.trim().to_lowercase();

        // User said no, terminate program
        if answer != "y" {
            break;
        }
    }

    
}

// Take in the 6502 assembly text file, splits it into different lines, and processes it with helper function
// Returns a Result<Vec<String>> (all the instructions in the text file)
fn read_script(filepath: PathBuf) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    // The vector with all the instructions from the script
    let mut instructions: Vec<String> = Vec::new();

    // Read text file
    let file = fs::File::open(&filepath)?;
    let parser = BufReader::new(file);
    for line in parser.lines() {
        match line {
            Ok(line) => {

                match process_line(&line) {
                    Ok(Some(hex_arr)) => {
                        // Add instructions from the line to our instructions vector
                        instructions.extend(hex_arr);
                    }
    
                    Ok(None) => {
                        // Do nothing, no valid instructions in this line
                    }
    
                    Err(_) => {
                        eprintln!("\nError processing line: {}", line);
                        std::process::exit(1);
                    }
                }
            }

            // Unexpected input
            Err(_) => {
                eprintln!("\nError!");
                std::process::exit(1);
            }
        }
        
    }

    Ok(instructions)
}

// Process each line character by character, checking for valid input, and then returning
// a Result<Option<Vec<String>> containing those valid hex pair inputs
fn process_line(line: &str) -> Result<Option<Vec<String>>, Box<dyn std::error::Error>> {
    let mut hex_arr: Vec<String> = Vec::new();
    let mut current_pair = String::new();

    // Line is empty, skip it
    if line.trim().is_empty() {
        return Ok(None);
    }

    for ch in line.chars() {
        // Check for comment in line, if so, skip it
        if ch == '/' {
            if current_pair.len() >= 1 {
                eprint!("\nInvalid input in line: {}.\n\nPlease don't put comments between hex digits\n", line);
                std::process::exit(1);
            }

            break;
        }

        else if ch.is_ascii_hexdigit() {
            // We only accept pairs of hex digits, so max pair length can only be 2
            if current_pair.len() >= 2 {
                eprintln!("\nInvalid input in line: {}.\n\nHex digits were not grouped in pairs or separated by a whitespace", line);
                std::process::exit(1);
            }

            // Add digit to pair
            current_pair.push(ch);
        }

        // Whitespace, make sure there aren't any whitespaces between single hex digits
        else if ch.is_whitespace() {
            if current_pair.len() != 0 {
                eprintln!("\nInvalid input in line: {}.\n\nSingle hex digits are not valid", line);
                std::process::exit(1);
            }
        }

        // Anything else
        else {
            eprintln!("\nInvalid input in line: {}", line);
            std::process::exit(1);
        }

        // We've found a pair, add it to our result array
        if current_pair.len() == 2 {
            hex_arr.push(current_pair.clone());
            current_pair.clear();
        }
    }

    Ok(Some(hex_arr))
}

// Takes in a vector of hex strings and converts it into a vector of hex u8, returning that
fn string_to_u8_hex(instructions: Vec<String>) -> Vec<u8>
{
    let mut program_vec: Vec<u8> = Vec::new();

    // Now we have to convert 
    for hex_pair in instructions {

        // Account for case insensitivity
        let lower_hex_pair = hex_pair.to_lowercase();
        let mut hex_result = 0;
        let mut power: u8 = 1;

          // Convert each array element into an actual, usable decimal value
        for digit in lower_hex_pair.chars() {
        
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

              // The last element of the array ends with \r\n, so this case is for when the code reaches that part
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

    program_vec
}