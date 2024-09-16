#![allow(non_snake_case)]
#![allow(dead_code)]
use core::panic;
use crate::opcode_info::OPCODES_TABLE;

const STACK_START: u16 = 0x0100; // Memory allocated for stack is 0100 - 01FF

/*
    Addressing Modes (we will arrange these in an enum):

    Absolute - Takes the entire address in little endian mode as an argument (2 or 1 byte(s))
    Zero Page - Takes an address in the first 255 bytes (1 byte)
    Immediate - Takes a value / constant as an argument (1 or 2 bytes)
    Implied - Takes no argument
    Indirect - Absolute Address that points to address with the instructions (2 bytes)
    Indexed Indirect -  Zero page address + x points to zero page address that has the target address
    Indirect Indexed - Fetches address from zero page address, adds y to fetched address to get address that contains target address
    Relative - 8-bit relative offset is added to program counter, used for branches

    Some can be modified with optional offsets from the x and y registers
*/    
pub enum AddressingMode {
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Accumulator,
    Immediate,
    Implied,
    IndexedIndirect,
    IndirectIndexed,
    Indirect,
    Relative,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
}

pub struct CPU {
    pub register_a: u8, // Accumulator register
    pub register_x: u8, // X index register
    pub register_y: u8, // Y index register
    pub status_flags: u8, // 0000_0000
    pub program_counter: u16, // Points to the next instruction to execute
    pub stack_pointer: u8, // Points to the top of the stack. The stack for the 6502 grows top to bottom. Memory allocated for stack pointer is 0x0100 - 0x01FF
    pub info: Vec<u16>, // To store our info after the program terminates
    pub print_mode: bool,
    ram: [u8; 0xFFFF]
}

impl CPU { 
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status_flags: 0b0000_0000,
            program_counter: 0,
            stack_pointer: 0xFF, 
            info: Vec::new(),
            print_mode: false,
            ram: [0; 0xFFFF]
        }
    }

/* 
    ---------------------------------------------------------------------------------------------------------
    FUNCTIONS FOR MEMORY

    We need these to fetch data from memory. Addresses are stored in little endian mode: least significant byte first, most significant byte second.
    If we want to fetch a 16-bit address, we have to keep that in mind
*/
    pub fn read_memory_u8(&self, address: u16) -> u8 {
        self.ram[address as usize]
    }

    pub fn write_memory_u8(&mut self, address: u16, data: u8) {
        self.ram[address as usize] = data;
    }

    pub fn read_memory_u16(&mut self, position: u16) -> u16 {
        let lsb = self.read_memory_u8(position) as u16;
        let msb = self.read_memory_u8(position + 1) as u16;
        (msb << 8) | (lsb as u16)
    }

    pub fn write_memory_u16(&mut self, position: u16, data: u16) {
        let msb = (data >> 8) as u8;
        let lsb = (data & 0xFF) as u8;
        self.write_memory_u8(position, lsb);
        self.write_memory_u8(position + 1, msb);
    }   

    /*
    ---------------------------------------------------------------------------------------------------------
    EXECUTION, LOADING, AND RESETTING PROGRAM

    Here are the functions that we need to load in our program, reset it, and execute it. The giant match function 
    gets an opcode, finds its matching function with its hex code, executes that function, and updates the program counter
    to get the next instruction
    */

    // Run instructions from the program ROM 
    pub fn callback<F>(&mut self, mut call: F) where F: FnMut(&mut CPU), {

        loop {
            
            let opcode = self.read_memory_u8(self.program_counter);
            let opcode_info = match OPCODES_TABLE.get(&opcode) {
                Some(info) => info,
                None => {
                    panic!("Error opcode is: {:X}", opcode);
                }
            };

            let mode = &opcode_info.mode;

            self.program_counter += 1;

            match opcode {
 
                0x69 | 0x65 | 0x75 | 0x6D | 0x7D | 0x79 | 0x61 | 0x71 => {
                    let address = self.get_address(mode);
                    let data = self.read_memory_u8(address);
                    self.ADC(data);
                }

                0x0A => self.ASL_ACCUMULATOR(),

                0x06 | 0x16 | 0x0E | 0x1E => {
                    self.ASL(mode);
                }

                0x29 | 0x25 | 0x35 | 0x2D | 0x3D | 0x39 | 0x21 | 0x31 => {
                    self.AND(mode);
                }

                0x24 | 0x2C => {
                    self.BIT(mode);
                }

                // BCC
                0x90 => self.BRANCH(self.status_flags & 0b0000_0001 == 0b0000_0000),

                // BCS 
                0xB0 => self.BRANCH(self.status_flags & 0b0000_0001 == 0b0000_0001),

                // BEQ 
                0xF0 => self.BRANCH(self.status_flags & 0b0000_0010 == 0b0000_0010),
                
                // BMI 
                0x30 => self.BRANCH(self.status_flags & 0b1000_0000 == 0b1000_0000),

                // BNE 
                0xD0 => self.BRANCH(self.status_flags & 0b0000_0010 == 0b0000_0000),
                
                // BPL
                0x10 => self.BRANCH(self.status_flags & 0b1000_0000 == 0b0000_0000),
                
                // BVC 
                0x50 => self.BRANCH(self.status_flags & 0b0100_0000 == 0b0000_0000),
                
                // BVS 
                0x70 => self.BRANCH(self.status_flags & 0b0100_0000 == 0b0100_0000),

                // BRK
                0x00 => {
                    if self.print_mode { // We don't want our values to be printed while playing snake!
                        self.save_and_print(&opcode_info.mnemonic, &opcode_info.hex_code); 
                    }
                    
                    return
                }

                // CLC
                0x18 => self.clear_carry_flag(),

                0xD8 => self.CLD(),

                // CLI
                0x58 => self.clear_interrupt_disable_flag(),
                
                // CLV
                0xB8 => self.clear_overflow_flag(),

                0xC9 | 0xC5 | 0xD5 | 0xCD | 0xDD | 0xD9 | 0xC1 | 0xD1 => {
                    self.COMPARE(mode, self.register_a);
                }

                0xE0 | 0xE4 | 0xEC => {
                    self.COMPARE(mode, self.register_x);
                }

                0xC0 | 0xC4 | 0xCC => {
                    self.COMPARE(mode, self.register_y);
                }
                
                0xC7 | 0xD7 | 0xCF | 0xDF | 0xDB | 0xC3 | 0xD3 => {
                    self.DCP(mode);
                }

                0xC6 | 0xD6 | 0xCE | 0xDE => {
                    self.DEC(mode);
                }

                0xCA => self.DEX(),

                0x88 => self.DEY(),

                0x49 | 0x45 | 0x55 | 0x4D | 0x5D | 0x59 | 0x41 | 0x51 => {
                    self.EOR(mode);
                }

                0xE6 | 0xF6 | 0xEE | 0xFE => {
                    self.INC(mode);
                }

                0xE8 => self.INX(),

                0xC8 => self.INY(),

                0x4C => self.JMP_ABSOLUTE(),

                0x6C => self.JMP_INDIRECT(),

                0x20 => self.JSR(),

                0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => {
                    self.LDA(mode);
                }

                0xA2 | 0xA6 | 0xB6 | 0xAE | 0xBE => {
                    self.LDX(mode);
                }

                0xA0 | 0xA4 | 0xB4 | 0xAC | 0xBC => { 
                    self.LDY(mode);
                }

                0x4A => self.LSR_ACCUMULATOR(),

                0x46 | 0x56 | 0x4E | 0x5E => {
                    self.LSR(mode);
                }

                // NOP
                0xEA | 0x80 | 0x04 | 0x44 | 0x64 => self.program_counter = self.program_counter + 1,

                0x09 | 0x05 | 0x15 | 0x0D | 0x1D | 0x19 | 0x01 | 0x11 => {
                    self.ORA(mode);
                }

                0x48 => self.PHA(),

                0x08 => self.PHP(),

                0x68 => self.PLA(),

                0x28 => self.PLP(),

                0x2A => self.ROL_ACCUMULATOR(),

                0x26 | 0x36 | 0x2E | 0x3E => {
                    self.ROL(mode);
                }

                0x6A => self.ROR_ACCUMULATOR(),

                0x66 | 0x76 | 0x6E | 0x7E => {
                    self.ROR(mode);
                }

                0x40 => self.RTI(),

                0x60 => self.RTS(),

                0xE9 | 0xE5 | 0xF5 | 0xED | 0xFD | 0xF9 | 0xE1 | 0xF1 => {
                    self.SBC(mode);
                }

                // SEC
                0x38 => self.set_carry_flag(),

                // SED
                0xF8 => self.SED(),

                // SEI
                0x78 => self.set_interrupt_disable_flag(),
                
                0x85 | 0x95 | 0x8D | 0x9D | 0x99 | 0x81 | 0x91 => {
                    self.STA(mode);
                }

                0x86 | 0x96 | 0x8E => {
                    self.STX(mode);
                }

                0x84 | 0x94 | 0x8C => {
                    self.STY(mode);
                }

                0xAA => self.TAX(),

                0xA8 => self.TAY(),
                
                0xBA => self.TSX(),
                
                0x8A => self.TXA(),
                
                0x9A => self.TXS(),
                
                0x98 => self.TYA(),
                
                _ => {
                    println!("{} is an invalid Opcode! Program terminated", &opcode_info.hex_code);
                    return;
                }
            }

            self.update_program_counter(&opcode);

            if self.print_mode { // We don't want our values to be printed while playing snake!
                self.save_and_print(&opcode_info.mnemonic, &opcode_info.hex_code); 
            }

            call(self); // Return to the function that called this function
        }
    }

    // Load into program ROM without executing it
    pub fn load(&mut self, program: &Vec<u8>) {
        self.ram[0x0600..(0x0600 + program.len())].copy_from_slice(&program[..]);
        self.write_memory_u16(0xFFFC, 0x0600);

        self.program_counter = self.read_memory_u16(0xFFFC); // 0xFFFC holds address of the starting instruction
    }

    // Load into program ROM and execute it, useful for testing
    pub fn load_and_execute(&mut self, program: Vec<u8>) {
        self.load(&program);

        // Clear registers in case user wants to load and execute another program afterwards
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.status_flags = 0b0000_0000;
        self.program_counter = self.read_memory_u16(0xFFFC); // 0xFFFC holds address of the starting instruction
        
        self.callback(|_| {});
        self.clear(&program);
    }

    // Program counter must be updated accordingly after every executed opcode
    pub fn update_program_counter(&mut self, opcode: &u8) {
        let opcode_info = OPCODES_TABLE.get(&opcode).unwrap();
        // Byte-length includes the opcode itself, which we don't want to include
        self.program_counter += (opcode_info.byte_length as u16) - 1;
    }

    // Unloads program from ROM
    pub fn clear(&mut self, program: &Vec<u8>) {
        for i in 0x0600..= 0x8000 + program.len() {
            self.ram[i] = 0;
        }
    }

    // Stores current processor info and prints it to the command line
    fn save_and_print(&mut self, mnemonic: &str, hexcode: &u8) {
        self.info = vec![self.register_a as u16, self.register_x as u16, self.register_y as u16, self.stack_pointer as u16, 
        self.status_flags as u16, self.program_counter - 1];
        
        println!("\n AFTER {} - {:#04x} \n \n Register Accumulator: {:#04x} \n Register X: {:#04x} \n Register Y: {:#04x} \n Stack Pointer: {} \n Status Flags: {:#b} \n Program Counter: {:#06x} \n", 
        mnemonic, hexcode, self.info[0], self.info[1], self.info[2], self.info[3], self.info[4], self.info[5]);
    }

/*
    ---------------------------------------------------------------------------------------------------------
    STACK OPERATIONS
    
    Since this is a downward growing stack, the stack pointer always points 
    to the next empty location in memory
*/

    pub fn pop_stack_u8(&mut self) -> u8 {
        // Start of the stack is at 0x01FF, so popping an item brings it closer to this address
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        self.read_memory_u8(STACK_START as u16 + self.stack_pointer as u16)
    }

    pub fn push_stack_u8(&mut self, data: u8) {
        // Similarly, pushing an item brings it further away from 0x01FF
        self.write_memory_u8(STACK_START as u16 + self.stack_pointer as u16, data);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    }

    pub fn pop_stack_u16(&mut self) -> u16 {
        // Push two bytes to the stack one at a time
        let lsb = self.pop_stack_u8() as u16;
        let msb: u16 = self.pop_stack_u8() as u16;

        (msb << 8) | lsb
    }

    pub fn push_stack_u16(&mut self, data: u16) {
        let msb = (data >> 8) as u8;
        let lsb = (data & 0xFF) as u8;

        self.push_stack_u8(msb);
        self.push_stack_u8(lsb);
    }

 /*
    ---------------------------------------------------------------------------------------------------------
    ADDRESSING MODES
    
    The addressing mode of an opcode determines how that opcode deals with data
*/

    fn get_address(&mut self, mode: &AddressingMode) -> u16 {
        match mode {

            AddressingMode::Absolute => self.read_memory_u16(self.program_counter), // Little endian mode, get full address

            AddressingMode::AbsoluteX => {
                // Read address from program counter and add the offset from register x to get the resulting address
                let base_address: u16 = self.read_memory_u16(self.program_counter);
                base_address.wrapping_add(self.register_x as u16)
            }

            AddressingMode::AbsoluteY => {
                // Same here but for register y
                let base_address: u16 = self.read_memory_u16(self.program_counter);
                base_address.wrapping_add(self.register_y as u16)
            }

            AddressingMode::Immediate | AddressingMode::Relative => self.program_counter, // Fetch the constant that comes after the opcode, no address needed

            AddressingMode::Implied | AddressingMode::Accumulator | AddressingMode::Indirect => {
                panic!("Mode does not require an argument!");
            }

            AddressingMode::IndexedIndirect => {

                // Get one-byte, add register x to it as an offset to get the lsb byte, what follows after is the msb byte. That is the address
                let byte: u8 = self.read_memory_u8(self.program_counter);
                
                let pointer: u8 = (byte).wrapping_add(self.register_x); 
                let lsb = self.read_memory_u8(pointer as u16);
                let msb = self.read_memory_u8(pointer.wrapping_add(1) as u16);
                
                (msb as u16) << 8 | (lsb as u16)
            }

            AddressingMode::IndirectIndexed => {
                let base_address = self.read_memory_u8(self.program_counter);
                
                let lsb = self.read_memory_u8(base_address as u16);
                let msb = self.read_memory_u8((base_address as u8).wrapping_add(1) as u16);
                let unadded_address = (msb as u16) << 8 | (lsb as u16);
                let added_address = unadded_address.wrapping_add(self.register_y as u16);
                
                added_address
            }

            // Address is located in the zero page, only one byte needed
            AddressingMode::ZeroPage => self.read_memory_u8(self.program_counter) as u16,

            // Get an address located in the zero page by adding the next byte with register x
            AddressingMode::ZeroPageX => {
                let position = self.read_memory_u8(self.program_counter);
                let address = position.wrapping_add(self.register_x) as u16;
                
                address 
            }

            // Do the same but with register y
            AddressingMode::ZeroPageY => {
                let position: u8 = self.read_memory_u8(self.program_counter);
                let address: u16 = position.wrapping_add(self.register_y) as u16;
                
                address
            }
        }
    }
    
/* 
    ---------------------------------------------------------------------------------------------------------
    STATUS FLAG FUNCTIONS 

    C: Carry Flag (LSB, bit 0) (Unsigned overflow)
    Z: Zero Flag 
    I: Interrupt Disable
    D: Decimal Mode Flag (Not used)
    U: Unused Flag
    B: Break Flag
    V: Overflow Flag (Signed overflow)
    N: Negative Flag (MSB, bit 7)

    N V B U D I Z C

    6502 uses zero-based index (0 to 7 bits)

    The status flag marks down certain conditions whenever they are met, such as when there is an overflow or zero result
*/
    fn set_overflow_flag(&mut self) {
        self.status_flags = self.status_flags | 0b0100_0000;
    }
    
    fn clear_overflow_flag(&mut self) {
        self.status_flags = self.status_flags & 0b1011_1111;
    }

    fn set_interrupt_disable_flag(&mut self) {
        self.status_flags = self.status_flags | 0b0000_0100;
    }
    
    fn clear_interrupt_disable_flag(&mut self) {
        self.status_flags = self.status_flags & 0b1111_1011;
    }

    fn set_carry_flag(&mut self) {
        self.status_flags = self.status_flags | 0b0000_0001;
    }
    
    fn clear_carry_flag(&mut self) {
        self.status_flags = self.status_flags & 0b1111_1110;
    }

    fn set_zero_flag(&mut self) {
        self.status_flags = self.status_flags | 0b0000_0010;
    }

    fn clear_zero_flag(&mut self) {
        self.status_flags = self.status_flags & 0b1111_1101;
    }

    fn set_break_flag(&mut self) {
        self.status_flags = self.status_flags | 0b0010_0000;
    }

    fn clear_break_flag(&mut self) {
        self.status_flags = self.status_flags & 0b1101_1111;
    }

    fn set_negative_flag(&mut self) {
        self.status_flags = self.status_flags | 0b1000_0000;
    }

    fn clear_negative_flag(&mut self) {
        self.status_flags = self.status_flags & 0b0111_1111;
    }

    fn zero_and_negative_flags(&mut self, result: u8) {
        // Zero flag
        if result == 0 {
            self.set_zero_flag();
        } else {
            self.clear_zero_flag();
        }

        // Negative flag
        if result & 0b1000_0000 != 0 {
            self.set_negative_flag();
        } else {
            self.clear_negative_flag();
        }
    }

/*
    ---------------------------------------------------------------------------------------------------------
    THE ACTUAL OPCODE FUNCTIONS

    From here on out, the functions will implement
    each of the 6502 opcodes, changing the status flags
    as appropriate 
*/
    // Add with carry
    fn ADC(&mut self, data: u8) {
        let carry = self.status_flags & 0b0000_0001;
        let result = self.register_a as u16 + data as u16 + carry as u16;
        
        // Detect unsigned overflow from the addition
        // And changing the carry flag accordingly 
        // ADC doesn't normally clear carry, but it saves headaches
        if result > 0xFF {
            self.set_carry_flag();
        } else {
            self.clear_carry_flag();
        }

        let sum = result as u8;

        // Computing signed overflow with this formula:
        // (Memory ^ result) & (accumulator ^ result) & 0x80 is nonzero
        if (data ^ sum) & (self.register_a ^ sum) & 0x80 != 0 {
            self.set_overflow_flag();
        } else {
            self.clear_overflow_flag();
        }

        self.register_a = sum;
        self.zero_and_negative_flags(self.register_a);
    }

    fn AND(&mut self, mode: &AddressingMode) {
        let address = self.get_address(mode);
        let data = self.read_memory_u8(address);
        self.register_a = self.register_a & data;

        self.zero_and_negative_flags(self.register_a);
    }

    // Arithmetic shift left with register a
    fn ASL_ACCUMULATOR(&mut self) {
        if self.register_a >> 7 == 1 {
            self.set_carry_flag();
        } else {
            self.clear_carry_flag();
        }

        self.register_a = self.register_a << 1;
        self.zero_and_negative_flags(self.register_a);
    }

    // General arithmetic shift left
    fn ASL(&mut self, mode: &AddressingMode) {
        let address = self.get_address(mode);
        let mut data = self.read_memory_u8(address);

        if data >> 7 == 1 {
            self.set_carry_flag();
        } else {
            self.clear_carry_flag();
        }

        data = data << 1;

        self.write_memory_u8(address, data);
        self.zero_and_negative_flags(data);
    }

    // Works like AND opcode, except it doesn't change register a so not really
    fn BIT(&mut self, mode: &AddressingMode) {
        let address = self.get_address(mode);
        let data = self.read_memory_u8(address);
        // Bits 7 and 6 of the value from memory are copied into the N and V flags
        if (data & 0b1000_0000) >> 7 == 1 {
            self.set_negative_flag();
        } else {
            self.clear_negative_flag();
        }

        if (data & 0b0100_0000) >> 6 == 1 {
            self.set_overflow_flag();
        } else {
            self.clear_overflow_flag()
        }

        let result = self.register_a & data;

        if result == 0 {
            self.set_zero_flag();
        } else {
            self.clear_zero_flag();
        }
    }

    // All of the branch instructions, which act as jumps depending on the status of the status flag
    fn BRANCH(&mut self, condition: bool) {
        // We branch starting from the instruction after the branch opcode
        if condition {
            let offset = self.read_memory_u8(self.program_counter) as i8;
            let jump_address = self.program_counter.wrapping_add(1).wrapping_add(offset as u16); // 0x00 means the very next instruction
            self.program_counter = jump_address - 1; // Since counter is incremented by one after this instruction
        }
    }

    // Clear decimal, I'm not sure why I put it in this section but oh well...
    fn CLD(&mut self) {
        self.status_flags = self.status_flags & 0b1111_0111;
    }

    // Compare the register with a value in memory and change the carry flag accordingly
    fn COMPARE(&mut self, mode: &AddressingMode, register: u8) {
        // Register a / x / y - memory
        let address: u16 = self.get_address(mode);
        let value = self.read_memory_u8(address);

        if register >= value {
            self.set_carry_flag();
        } else {
            self.clear_carry_flag();
        }

        self.zero_and_negative_flags(register.wrapping_sub(value));
    }

    // DEC + CMP opcode, not really official
    fn DCP(&mut self, mode: &AddressingMode) {
        self.DEC(&mode);
        self.COMPARE(&mode, self.register_a);
    }

    // Decrement a value in memory by 1
    fn DEC(&mut self, mode: &AddressingMode) {
        let address = self.get_address(mode);
        let result = self.read_memory_u8(address).wrapping_sub(1);
        self.ram[address as usize] = result;
        self.zero_and_negative_flags(result)
    }

    // These two decrement their registers by 1 respectively
    fn DEX(&mut self) {
        self.register_x = self.register_x.wrapping_sub(1);
        self.zero_and_negative_flags(self.register_x);
    }

    fn DEY(&mut self) {
        self.register_y = self.register_y.wrapping_sub(1);
        self.zero_and_negative_flags(self.register_y);
    }

    // Exclusive or operation
    fn EOR(&mut self, mode: &AddressingMode) {
        let address = self.get_address(mode);
        let data = self.read_memory_u8(address);
        self.register_a = self.register_a ^ data;
        self.zero_and_negative_flags(self.register_a);
    }

    // Increment value in memory by 1, same for registers x and y
    fn INC(&mut self, mode: &AddressingMode) {
        let address = self.get_address(mode);
        let result = self.read_memory_u8(address).wrapping_add(1);
        self.ram[address as usize] = result;
        self.zero_and_negative_flags(result)
    }

    fn INX(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.zero_and_negative_flags(self.register_x);
    }

    fn INY(&mut self) {
        self.register_y = self.register_y.wrapping_add(1);
        self.zero_and_negative_flags(self.register_y);
    }

    // Jump to a location in memory
    fn JMP_ABSOLUTE(&mut self) {
        let specified_address = self.read_memory_u16(self.program_counter);
        self.program_counter = specified_address - 2; // Since the counter is incremented by two after this
    }

    fn JMP_INDIRECT(&mut self) {
        let address = self.read_memory_u16(self.program_counter);

        /*
        6502 has a bug where it doesn't correctly fetch
        the target address if it falls on a page boundary
        (we'll emulate that as well)
        */
        let indirect_reference = if (address & 0x00FF) == 0x00FF {
            let lsb = self.read_memory_u8(address);
            let msb = self.read_memory_u8(address & 0xFF00);

            (msb as u16) << 8 | (lsb as u16)
        } else {
            self.read_memory_u16(address)
        };

        self.program_counter = indirect_reference - 2; // Since the counter is incremented by two after this
    }

    // Jump to a subroutine
    fn JSR(&mut self) {
        self.push_stack_u16(self.program_counter + 2); // Location of JSR opcode
        let target_address = self.read_memory_u16(self.program_counter);
        self.program_counter = target_address - 2; // JSR byte length is 3 (counter jumps forward by 2), so it must be negated
    }

    // Load values into the a, x, and y registers
    fn LDA(&mut self, mode: &AddressingMode) {
        let address = self.get_address(mode);
        self.register_a = self.read_memory_u8(address);
        self.zero_and_negative_flags(self.register_a);
    }

    fn LDX(&mut self, mode: &AddressingMode) {
        let address: u16 = self.get_address(mode);
        self.register_x = self.read_memory_u8(address);
        self.zero_and_negative_flags(self.register_x);
    }

    fn LDY(&mut self, mode: &AddressingMode) {
        let address: u16 = self.get_address(mode);
        self.register_y = self.read_memory_u8(address);
        self.zero_and_negative_flags(self.register_y);
    }

    // Logical shift right and logical shift left functions, with unique functions for the accumulator itself
    fn LSR_ACCUMULATOR(&mut self) {
        // Data shifted to the right. Old bit 0 is carry flag
        // New bit 7 is set to 0
        let old_bit_zero =  self.register_a & 0b0000_0001;

        self.register_a = (self.register_a >> 1) & 0b0111_1111;

        if old_bit_zero == 0 {
            self.clear_carry_flag();
        } else {
            self.set_carry_flag();
        }

        self.zero_and_negative_flags(self.register_a);
    }

    fn LSR(&mut self, mode: &AddressingMode) {
        // Data shifted to the right. Old bit 0 is carry flag
        // New bit 7 is set to 0
        let address = self.get_address(mode);
        let mut data = self.read_memory_u8(address);
        let old_bit_zero =  data & 0b0000_0001;

        data = (data >> 1) & 0b0111_1111;

        if old_bit_zero == 0 {
            self.clear_carry_flag();
        } else {
            self.set_carry_flag();
        }

        self.write_memory_u8(address, data);
        self.zero_and_negative_flags(data);
    }

    // Or operation with the accumulator
    fn ORA(&mut self, mode: &AddressingMode) {
        let address = self.get_address(mode);
        let data = self.read_memory_u8(address);
        self.register_a = self.register_a | data;
        self.zero_and_negative_flags(self.register_a);
    }

    // Push accumulator onto stack
    fn PHA(&mut self) {
        self.push_stack_u8(self.register_a);
    }

    // Push status flag onto stack
    fn PHP(&mut self) {
        // Break is pushed as 1
        self.set_break_flag();
        self.push_stack_u8(self.status_flags);
    }

    // The alternatives for pulling the accumulator and the status flag from the stack
    fn PLA(&mut self) {
        self.register_a = self.pop_stack_u8();
        self.zero_and_negative_flags(self.register_a);
    }

    fn PLP(&mut self) {
        // Break discarded
        self.status_flags = self.pop_stack_u8();
        self.clear_break_flag();
    }

    // Rotate value in accumulator to the right
    fn ROL_ACCUMULATOR(&mut self) {
        let old_bit_seven = (self.register_a & 0b1000_0000) >> 7;
        let current_carry_flag = self.status_flags & 0b0000_0001;

        self.register_a = self.register_a << 1;

        // Bit 0 is filled with the current carry flag value
        // Old bit 7 becomes new carry flag value
        if current_carry_flag == 0 {
            self.register_a = self.register_a & 0b1111_1110;
        } else {
            self.register_a = self.register_a | 0b0000_0001;
        }

        if old_bit_seven == 0 {
            self.clear_carry_flag();
        } else {
            self.set_carry_flag();
        }

        self.zero_and_negative_flags(self.register_a);
    }

    // Rotate to the left
    fn ROL(&mut self, mode: &AddressingMode) {
        let address = self.get_address(mode);
        let mut data = self.read_memory_u8(address);
        let old_bit_seven = (data & 0b1000_0000) >> 7;
        let current_carry_flag = self.status_flags & 0b0000_0001;

        data = data << 1;

        // Bit 0 is filled with the current carry flag value
        // Old bit 7 becomes new carry flag value
        if current_carry_flag == 0 {
            data = data & 0b1111_1110;
        } else {
            data = data | 0b0000_0001;
        }

        if old_bit_seven == 0 {
            self.clear_carry_flag();
        } else {
            self.set_carry_flag();
        }

        self.zero_and_negative_flags(data);
        self.write_memory_u8(address, data);
    }

    // Rotate accumulator value to the right
    fn ROR_ACCUMULATOR(&mut self) {
        let old_bit_seven = (self.register_a & 0b1000_0000) >> 7;
        let current_carry_flag = self.status_flags & 0b0000_0001;

        self.register_a = self.register_a >> 1;

        // Bit 0 is filled with the current carry flag value
        // Old bit 7 becomes new carry flag value
        if current_carry_flag == 0 {
            self.register_a = self.register_a & 0b1111_1110;
        } else {
            self.register_a = self.register_a | 0b0000_0001;
        }

        if old_bit_seven == 0 {
            self.clear_carry_flag();
        } else {
            self.set_carry_flag();
        }

        self.zero_and_negative_flags(self.register_a);
    }

    // Rotate value in memory to the right
    fn ROR(&mut self, mode: &AddressingMode) {
        let address = self.get_address(&mode);
        let mut data = self.read_memory_u8(address);
        let old_bit_seven = (data & 0b1000_0000) >> 7;
        let current_carry_flag = self.status_flags & 0b0000_0001;

        data = data >> 1;

        // Bit 0 is filled with the current carry flag value
        // Old bit 7 becomes new carry flag value
        if current_carry_flag == 0 {
            data = data & 0b1111_1110;
        } else {
            data = data | 0b0000_0001;
        }

        if old_bit_seven == 0 {
            self.clear_carry_flag();
        } else {
            self.set_carry_flag();
        }

        self.zero_and_negative_flags(data);
        self.write_memory_u8(address, data);
    }

    // Return from interrupt
    fn RTI(&mut self) {
        // Pulls flags followed by counter
        self.status_flags = self.pop_stack_u8();
        self.program_counter = self.pop_stack_u16();

        // Break discarded 
        self.clear_break_flag();
    }

    // Return from subroutine
    fn RTS(&mut self) {
        self.program_counter = self.pop_stack_u16(); 
    }

    // Subtract with carry 
    fn SBC(&mut self, mode: &AddressingMode) {
        // We simply take the two's complement
        // And call our ADC opcode (the borrow value will be added there)
        let address = self.get_address(&mode);
        let data = self.read_memory_u8(address);

        // A = A + NOT M + C (carry is added in ADC)
        self.ADC((data as i8).wrapping_neg() as u8);
    }   

    // Set the decimal flag, even though we're never going to use this...
    fn SED(&mut self) {
        self.status_flags = self.status_flags | 0b0000_1000;
    }

    // Save the value of a register into a memory address
    fn STA(&mut self, mode: &AddressingMode) {
        let address = self.get_address(mode);
        self.write_memory_u8(address, self.register_a);
    }

    fn STX(&mut self, mode: &AddressingMode) {
        let address: u16 = self.get_address(mode);
        self.write_memory_u8(address, self.register_x);
    }

    fn STY(&mut self, mode: &AddressingMode) {
        let address: u16 = self.get_address(mode);
        self.write_memory_u8(address, self.register_y);
    }

    // Transfer a value from one register to the other
    fn TAX(&mut self) {
        self.register_x = self.register_a;
        self.zero_and_negative_flags(self.register_x);
    }  

    fn TAY(&mut self) {
        self.register_y = self.register_a;
        self.zero_and_negative_flags(self.register_y);
    }

    fn TSX(&mut self) {
        self.register_x = self.stack_pointer;
        self.zero_and_negative_flags(self.register_x);
    }

    fn TXA(&mut self) {
        self.register_a = self.register_x;
        self.zero_and_negative_flags(self.register_a);
    }

    fn TXS(&mut self) {
        self.stack_pointer = self.register_x;
    }

    fn TYA(&mut self) {
        self.register_a = self.register_y;
        self.zero_and_negative_flags(self.register_a);
    }
}
