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
// + 1 means that there is an extra cycle if page crossed
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

        Opcode::new("LDX", AddressingMode::Immediate, 0xA2, 2, 2),
        Opcode::new("LDX", AddressingMode::ZeroPage, 0xA6, 2, 3),
        Opcode::new("LDX", AddressingMode::ZeroPageY, 0xB6, 2, 4),
        Opcode::new("LDX", AddressingMode::Absolute, 0xAE, 3, 4),
        Opcode::new("LDX", AddressingMode::AbsoluteY, 0xBE, 3, 4), // + 1

        Opcode::new("LDY", AddressingMode::Immediate, 0xA0, 2, 2),
        Opcode::new("LDY", AddressingMode::ZeroPage, 0xA4, 2, 2),
        Opcode::new("LDY", AddressingMode::ZeroPageX, 0xB4, 2, 4),
        Opcode::new("LDY", AddressingMode::Absolute, 0xAC, 3, 4),
        Opcode::new("LDY", AddressingMode::AbsoluteX, 0xBC, 3, 4), // + 1

        Opcode::new("SEC", AddressingMode::Implied, 0x38, 1, 2),
        Opcode::new("SED", AddressingMode::Implied, 0xF8, 1, 2),
        Opcode::new("SEI", AddressingMode::Implied, 0x78, 1, 2),

        Opcode::new("STA", AddressingMode::ZeroPage, 0x85, 2, 3),
        Opcode::new("STA", AddressingMode::ZeroPageX, 0x95, 2, 4),
        Opcode::new("STA", AddressingMode::Absolute, 0x8D, 3, 4),
        Opcode::new("STA", AddressingMode::AbsoluteX, 0x9D, 3, 5),
        Opcode::new("STA", AddressingMode::AbsoluteY, 0x99, 3, 5),
        Opcode::new("STA", AddressingMode::IndexedIndirect, 0x81, 2, 6),
        Opcode::new("STA", AddressingMode::IndirectIndexed, 0x91, 2, 6),

        Opcode::new("STX", AddressingMode::ZeroPage, 0x86, 2, 3),
        Opcode::new("STX", AddressingMode::ZeroPageY, 0x96, 2, 4),
        Opcode::new("STX", AddressingMode::Absolute, 0x8E, 3, 4),

        Opcode::new("STY", AddressingMode::ZeroPage, 0x84, 2, 3),
        Opcode::new("STY", AddressingMode::ZeroPageX, 0x94, 2, 4),
        Opcode::new("STY", AddressingMode::Absolute, 0x8C, 3, 4),

        Opcode::new("TAX", AddressingMode::Implied, 0xAA, 1, 2),
        Opcode::new("TAY", AddressingMode::Implied, 0xA8, 1, 2),
        Opcode::new("TSX", AddressingMode::Implied, 0xBA, 1, 2),
        Opcode::new("TXA", AddressingMode::Implied, 0x8A, 1, 2),
        Opcode::new("TXS", AddressingMode::Implied, 0x9A, 1, 2),
        Opcode::new("TYA", AddressingMode::Implied, 0x98, 1, 2)
    ];

    pub static ref OPCODES_TABLE: HashMap<u8, &'static Opcode> = {
        let mut table = HashMap::new();
        for opcode in &*OPCODES_LIST {
            table.insert(opcode.hex_code, opcode);
        }

        table
    };
}