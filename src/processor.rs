use crate::opcode_info::{self, OPCODES_TABLE};
use std::collections::HashMap;

/*
   For the CPU component (also known as the 2A03 chip in the case of the NES :D):

   1. Fetch next execution instruction from the instruction memory (using the program counter)
   2. Decode the instruction
   3. Execute the instruction
   4. Repeat the cycle (wait for the next clock signal)
*/

#[allow(dead_code)]
pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status_flags: u8,
    pub program_counter: u16,
    ram: [u8; 0xFFFF]
}

    /*
    Addressing Modes:

    Absolute - Takes the entire address as an argument (2 or 1 byte(s))
    Zero Page - Takes an address in the first 255 bytes (1 byte)
    Immediate - Takes a value as an argument (1 or 2 bytes)
    Implied - Takes no argument
    Indirect - Absolute Address that points to address with the instructions (2 bytes)
    Indexed Indirect -  Zero page address + x points to zero page address that has the target address
    Indirect Indexed - Fetches address from zero page address, adds y to fetched address to get address that contains target address
    
    Some can be modified with optional offsets from the x and y registers
    */    

#[allow(dead_code)]
pub enum AddressingMode {
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Immediate,
    Implied,
    IndexedIndirect,
    IndirectIndexed,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    None
}

pub trait Memory {
    fn read_memory(&self, address: u16) -> u8;

    fn write_memory(&mut self, address: u16, data: u8);

    /* 
    Addresses are stored in little endian mode: lsb first, msb second.
    If we want to fetch an address, we have to keep that in mind
    */
    fn read_memory_u16(&mut self, position: u16) -> u16 {
        let lsb = self.read_memory(position) as u16;
        let msb = self.read_memory(position + 1) as u16;
        (msb << 8) | (lsb as u16)
    }

    fn write_memory_u16(&mut self, position: u16, data: u16) {
        let msb = (data >> 8) as u8;
        let lsb = (data & 0xFF) as u8;
        self.write_memory(position, lsb);
        self.write_memory(position + 1, msb);
    }    
}

impl Memory for CPU {
    fn read_memory(&self, address: u16) -> u8 {
        self.ram[address as usize]
    }
    fn write_memory(&mut self, address: u16, data: u8) {
        self.ram[address as usize] = data;
    }
}

#[allow(dead_code)]
impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status_flags: 0,
            program_counter: 0,
            ram: [0; 0xFFFF]
        }
    }

    // Run instructions in the program ROM section
    pub fn run(&mut self) {
        loop {
            let opcode = self.read_memory(self.program_counter);
            self.program_counter += 1;

            match opcode {

                // BRK
                0x00 => return,

                // INX
                0xE8 => {
                    self.INX();
                }

                // LDA
                0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => {
                    let argument = self.read_memory(self.program_counter); 
                    self.program_counter += 1;
                    self.LDA();
                }

                // STA
                0x85 | 0x95 | 0x8D | 0x9D | 0x99 | 0x81 | 0x91 => {

                }

                // TAX
                0xAA => { 
                    self.TAX();
                }

                _ => todo!(),
            }
        }
    }

    // Load into program ROM
    pub fn load_program(&mut self, program: Vec<u8>) {
        self.ram[0x8000..(0x8000 + program.len())].copy_from_slice(&program[..]);
        self.write_memory_u16(0xFFFC, 0x8000);
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load_program(program);
        self.reset(); // Make sure no data from any previous program carries over
        self.run();
    }

    // Clear registers
    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.status_flags = 0;
        self.program_counter = self.read_memory_u16(0xFFFC);
    }

    fn operand_address(&mut self, mode: &AddressingMode) -> u16 {
        match mode {

            AddressingMode::Absolute => self.read_memory_u16(self.program_counter),

            AddressingMode::AbsoluteX => {
                let base_address: u16 = self.read_memory_u16(self.program_counter);
                let address: u16 = base_address.wrapping_add(self.register_x as u16);
                
                address
            }

            AddressingMode::AbsoluteY => {
                let base_address: u16 = self.read_memory_u16(self.program_counter);
                let address: u16 = base_address.wrapping_add(self.register_y as u16);
                
                address
            }

            AddressingMode::Immediate => self.program_counter,

            AddressingMode::Implied | AddressingMode::None => {
                panic!("Mode does not require an argument / is not supported!");
            }

            AddressingMode::IndexedIndirect => {
                let base_address = self.read_memory(self.program_counter);
                
                let pointer: u8 = (base_address as u8).wrapping_add(self.register_x); 
                let lsb = self.read_memory(pointer as u16);
                let msb = self.read_memory(pointer.wrapping_add(1) as u16);
                
                (msb as u16) << 8 | (lsb as u16)
            }

            AddressingMode::IndirectIndexed => {
                let base_address = self.read_memory(self.program_counter);
                
                let lsb = self.read_memory(base_address as u16);
                let msb = self.read_memory((base_address as u8).wrapping_add(1) as u16);
                let unadded_address = (msb as u16) << 8 | (lsb as u16);
                let added_address = unadded_address.wrapping_add(self.register_y as u16);
                
                added_address
            }

            AddressingMode::ZeroPage => self.read_memory(self.program_counter) as u16,

            AddressingMode::ZeroPageX => {
                let position = self.read_memory(self.program_counter);
                let address = position.wrapping_add(self.register_x) as u16;
                
                address 
            }

            AddressingMode::ZeroPageY => {
                let position: u8 = self.read_memory(self.program_counter);
                let address: u16 = position.wrapping_add(self.register_y) as u16;
                
                address
            }
        }
    }
    
    /* 
    The Status Flags:

    C: Carry Flag (LSB) 
    Z: Zero Flag 
    I: Interrupt Disable
    D: Decimal Mode Flag
    U: Unused Flag
    B: Break Command
    V: Overflow Flag
    N: Negative Flag (MSB)

    N V B U D I Z C
    */
    fn set_overflow_flag(&mut self) {
        self.status_flags = self.status_flags | 0b0100_0000;
    }
    
    fn clear_overflow_flag(&mut self) {
        self.status_flags = self.status_flags & 0b1011_1111;
    }

    fn zero_and_negative_flags(&mut self, result: u8) {
        // Zero flag
        if result == 0 {
            self.status_flags = self.status_flags | 0b0000_0010;
        } else {
            self.status_flags = self.status_flags & 0b1111_1101;
        }

        // Negative flag
        if result & 0b1000_0000 != 0 {
            self.status_flags = self.status_flags | 0b1000_0000;
        } else {
            self.status_flags = self.status_flags & 0b0111_1111;
        }
    }

    fn INX(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.zero_and_negative_flags(self.register_x);
        if self.register_x == 0 {
            self.set_overflow_flag();
        } else {
            self.clear_overflow_flag();
        }
    }

    fn LDA(&mut self, mode: &AddressingMode) {
        let address = self.operand_address(mode);
        self.register_a = self.read_memory(address);
        self.zero_and_negative_flags(self.register_a);
    }

    fn STA(&mut self, mode: &AddressingMode) {
        let address = self.operand_address(mode);
        self.write_memory(address, self.register_a);
    }

    fn TAX(&mut self) {
        self.register_x = self.register_a;
        self.zero_and_negative_flags(self.register_x);
    }

    
}
