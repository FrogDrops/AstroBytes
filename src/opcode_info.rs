use crate::processor::AddressingMode;
use std::collections::HashMap;
use lazy_static::lazy_static;

/*
Code - The byte that references the opcode
Mnemonic - The name of the opcode
Mode - Addressing mode
Cycles - Cycles needed to perform the opcode
Length - Length of the instruction (opcode + arguments)

The 6502 Reference from nesdev was used to help me make this! Great stuff there!
*/

pub struct Opcode {
    pub mnemonic: &'static str,
    pub mode: AddressingMode,
    pub hex_code: u8,
    pub byte_length: u8,
    pub num_cycles: u8
}

impl Opcode {
    fn new(mnemonic: &'static str, mode: AddressingMode, hex_code: u8, byte_length: u8, num_cycles: u8) -> Self {
        Opcode {
            mnemonic: mnemonic,
            mode: mode,
            hex_code,
            byte_length,
            num_cycles
        }    
    }
}

// With lazy static, data is deferred until it is first accessed 
lazy_static! {
    pub static ref OPCODES_LIST: Vec<Opcode> = vec![
        Opcode::new("BRK", AddressingMode::Implied, 0x00, 1, 7),

        Opcode::new("INX", AddressingMode::Implied, 0xE8, 1, 2),

        Opcode::new("LDA", AddressingMode::Immediate, 0xA9, 2, 2),
        Opcode::new("LDA", AddressingMode::ZeroPage, 0xA5, 2, 3),
        Opcode::new("LDA", AddressingMode::ZeroPageX, 0xB5, 2, 4),
        Opcode::new("LDA", AddressingMode::Absolute, 0xAD, 3, 4),
        Opcode::new("LDA", AddressingMode::AbsoluteX, 0xBD, 3, 4), // + 1
        Opcode::new("LDA", AddressingMode::AbsoluteY, 0xB9, 3, 4), // + 1
        Opcode::new("LDA", AddressingMode::IndexedIndirect, 0xA1, 2, 6),
        Opcode::new("LDA", AddressingMode::IndirectIndexed, 0xB1, 2, 5), // + 1

        Opcode::new("STA", AddressingMode::ZeroPage, 0x85, 2, 3),
        Opcode::new("STA", AddressingMode::ZeroPageX, 0x95, 2, 4),
        Opcode::new("STA", AddressingMode::Absolute, 0x8D, 3, 4),
        Opcode::new("STA", AddressingMode::AbsoluteX, 0x9D, 3, 5),
        Opcode::new("STA", AddressingMode::AbsoluteY, 0x99, 3, 5),
        Opcode::new("STA", AddressingMode::IndexedIndirect, 0x81, 2, 6),
        Opcode::new("STA", AddressingMode::IndirectIndexed, 0x91, 2, 6),

        Opcode::new("TAX", AddressingMode::Implied, 0xAA, 1, 2),
    ];

    pub static ref OPCODES_TABLE: HashMap<u8, &'static Opcode> = {
        let mut table = HashMap::new();
        for opcode in &*OPCODES_LIST {
            table.insert(opcode.hex_code, opcode);
        }

        table
    };
}