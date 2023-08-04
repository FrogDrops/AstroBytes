#![allow(non_snake_case)]
#![allow(dead_code)]
use crate::opcode_info::OPCODES_TABLE;

/*
   For the CPU component (also known as the 2A03 chip in the case of the NES :D):

   1. Fetch next execution instruction from the instruction memory (using the program counter)
   2. Decode the instruction
   3. Execute the instruction
   4. Repeat the cycle (wait for the next clock signal)
*/

pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status_flags: u8,
    pub program_counter: u16,
    pub stack_pointer: u8,
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
    fn read_memory_u8(&self, address: u16) -> u8;

    fn write_memory_u8(&mut self, address: u16, data: u8);

    /* 
    Addresses are stored in little endian mode: lsb first, msb second.
    If we want to fetch an address, we have to keep that in mind
    */
    fn read_memory_u16(&mut self, position: u16) -> u16 {
        let lsb = self.read_memory_u8(position) as u16;
        let msb = self.read_memory_u8(position + 1) as u16;
        (msb << 8) | (lsb as u16)
    }

    fn write_memory_u16(&mut self, position: u16, data: u16) {
        let msb = (data >> 8) as u8;
        let lsb = (data & 0xFF) as u8;
        self.write_memory_u8(position, lsb);
        self.write_memory_u8(position + 1, msb);
    }    
}

impl Memory for CPU {
    fn read_memory_u8(&self, address: u16) -> u8 {
        self.ram[address as usize]
    }
    fn write_memory_u8(&mut self, address: u16, data: u8) {
        self.ram[address as usize] = data;
    }
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status_flags: 0,
            program_counter: 0,
            stack_pointer: 0xFF,
            ram: [0; 0xFFFF]
        }
    }

    // Run instructions in the program ROM section
    pub fn execute(&mut self) {

        loop {
            let opcode = self.read_memory_u8(self.program_counter);
            let opcode_info = OPCODES_TABLE.get(&opcode).unwrap();
            let mode = &opcode_info.mode;

            self.program_counter += 1;

            match opcode {
                // BRK
                0x00 => return,

                0x18 => self.CLC(),

                0xD8 => self.CLD(),

                0x58 => self.CLI(),

                0xB8 => self.CLV(),

                0xC9 | 0xC5 | 0xD5 | 0xCD | 0xDD | 0xD9 | 0xC1 | 0xD1 => {
                    self.COMPARE(mode, self.register_a);
                }

                0xE0 | 0xE4 | 0xEC => {
                    self.COMPARE(mode, self.register_x);
                }

                0xC0 | 0xC4 | 0xCC => {
                    self.COMPARE(mode, self.register_y);
                }

                0xC6 | 0xD6 | 0xCE | 0xDE => {
                    self.DEC(mode);
                }

                // DEX
                0xCA => self.DEX(),

                // DEY
                0x88 => self.DEY(),

                // INC
                0xE6 | 0xF6 | 0xEE | 0xFE => {
                    self.INC(mode);
                }

                // INX
                0xE8 => self.INX(),

                // INY
                0xC8 => self.INY(),

                // LDA
                0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => {
                    self.LDA(mode);
                }

                // LDX
                0xA2 | 0xA6 | 0xB6 | 0xAE | 0xBE => {
                    self.LDX(mode);
                }

                // LDY
                0xA0 | 0xA4 | 0xB4 | 0xAC | 0xBC => { 
                    self.LDY(mode);
                }

                // SEC
                0x38 => self.SEC(),

                // SED
                0xF8 => self.SED(),

                // SEI
                0x78 => self.SEI(),
                
                // STA
                0x85 | 0x95 | 0x8D | 0x9D | 0x99 | 0x81 | 0x91 => {
                    self.STA(mode);
                }

                // STX
                0x86 | 0x96 | 0x8E => {
                    self.STX(mode);
                }

                // STY
                0x84 | 0x94 | 0x8C => {
                    self.STY(mode);
                }

                // TAX
                0xAA => self.TAX(),

                // TAY
                0xA8 => self.TAY(),
                
                // TSX
                0xBA => self.TSX(),
                
                // TXA
                0x8A => self.TXA(),
                
                // TXS
                0x9A => self.TXS(),
                
                // TYA
                0x98 => self.TYA(),
                
                _ => todo!(),
            }

            self.update_program_counter(&opcode);
        }
    }

    // Load into program ROM
    pub fn load_program(&mut self, program: &Vec<u8>) {
        self.ram[0x8000..(0x8000 + program.len())].copy_from_slice(&program[..]);
        self.write_memory_u16(0xFFFC, 0x8000);
    }

    pub fn load_and_execute(&mut self, program: Vec<u8>) {
        self.load_program(&program);
        self.reset(); // Make sure no data from any previous program carries over
        self.execute();
        self.clear_program(&program);
    }

    // Clear registers
    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.status_flags = 0;
        self.program_counter = self.read_memory_u16(0xFFFC);
    }

    // Program counter must be updated accordingly after every executed opcode
    pub fn update_program_counter(&mut self, opcode: &u8) {
        let opcode_info = OPCODES_TABLE.get(&opcode).unwrap();
        // Byte-length includes the opcode itself, which we don't want to include
        self.program_counter += (opcode_info.byte_length as u16) - 1;
    }

    pub fn clear_program(&mut self, program: &Vec<u8>) {
        for i in 0x8000..= 0x8000 + program.len() {
            self.ram[i] = 0;
        }
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
                let base_address = self.read_memory_u8(self.program_counter);
                
                let pointer: u8 = (base_address as u8).wrapping_add(self.register_x); 
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

            AddressingMode::ZeroPage => self.read_memory_u8(self.program_counter) as u16,

            AddressingMode::ZeroPageX => {
                let position = self.read_memory_u8(self.program_counter);
                let address = position.wrapping_add(self.register_x) as u16;
                
                address 
            }

            AddressingMode::ZeroPageY => {
                let position: u8 = self.read_memory_u8(self.program_counter);
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

    fn CLC(&mut self) {
        self.clear_carry_flag();
    }

    fn CLD(&mut self) {
        self.status_flags = self.status_flags & 0b1111_0111;
    }

    fn CLI(&mut self) {
        self.clear_interrupt_disable_flag();
    }

    fn CLV(&mut self) {
        self.clear_overflow_flag();
    }

    fn COMPARE(&mut self, mode: &AddressingMode, register: u8) {
        // Register a / x / y - memory
        let address: u16 = self.operand_address(mode);
        let value = self.read_memory_u8(address);

        if register >= value {
            self.set_carry_flag();
        } else {
            self.clear_carry_flag();
        }

        self.zero_and_negative_flags(register.wrapping_sub(value));
    }

    fn DEC(&mut self, mode: &AddressingMode) {
        let address = self.operand_address(mode);
        let result = self.read_memory_u8(address).wrapping_sub(1);
        self.ram[address as usize] = result;
        self.zero_and_negative_flags(result)
    }

    fn DEX(&mut self) {
        self.register_x = self.register_x.wrapping_sub(1);
        self.zero_and_negative_flags(self.register_y);
    }

    fn DEY(&mut self) {
        self.register_y = self.register_y.wrapping_sub(1);
        self.zero_and_negative_flags(self.register_y);
    }

    fn INC(&mut self, mode: &AddressingMode) {
        let address = self.operand_address(mode);
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

    fn LDA(&mut self, mode: &AddressingMode) {
        let address = self.operand_address(mode);
        self.register_a = self.read_memory_u8(address);
        self.zero_and_negative_flags(self.register_a);
    }

    fn LDX(&mut self, mode: &AddressingMode) {
        let address: u16 = self.operand_address(mode);
        self.register_x = self.read_memory_u8(address);
        self.zero_and_negative_flags(self.register_x);
    }

    fn LDY(&mut self, mode: &AddressingMode) {
        let address: u16 = self.operand_address(mode);
        self.register_y = self.read_memory_u8(address);
        self.zero_and_negative_flags(self.register_y);
    }

    fn SEC(&mut self) {
        self.set_carry_flag();
    }

    fn SED(&mut self) {
        self.status_flags = self.status_flags | 0b0000_1000;
    }

    fn SEI(&mut self) {
        self.set_interrupt_disable_flag();
    }

    fn STA(&mut self, mode: &AddressingMode) {
        let address = self.operand_address(mode);
        self.write_memory_u8(address, self.register_a);
    }

    fn STX(&mut self, mode: &AddressingMode) {
        let address: u16 = self.operand_address(mode);
        self.write_memory_u8(address, self.register_x);
    }

    fn STY(&mut self, mode: &AddressingMode) {
        let address: u16 = self.operand_address(mode);
        self.write_memory_u8(address, self.register_y);
    }

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
