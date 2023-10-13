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
        Opcode::new("AND", AddressingMode::Immediate, 0x29, 2, 2),
        Opcode::new("AND", AddressingMode::ZeroPage, 0x25, 2, 3),
        Opcode::new("AND", AddressingMode::ZeroPageX, 0x35, 2, 4),
        Opcode::new("AND", AddressingMode::Absolute, 0x2D, 3, 4),
        Opcode::new("AND", AddressingMode::AbsoluteX, 0x3D, 3, 4), // + 1
        Opcode::new("AND", AddressingMode::AbsoluteY, 0x39, 3, 4), // + 1
        Opcode::new("AND", AddressingMode::IndexedIndirect, 0x21, 2, 6),
        Opcode::new("AND", AddressingMode::IndirectIndexed, 0x31, 2, 5), // + 1

        Opcode::new("ASL", AddressingMode::Accumulator, 0x0A, 1, 2),
        Opcode::new("ASL", AddressingMode::ZeroPage, 0x06, 2, 5),
        Opcode::new("ASL", AddressingMode::ZeroPageX, 0x16, 2, 6),
        Opcode::new("ASL", AddressingMode::Absolute, 0x0E, 3, 6),
        Opcode::new("ASL", AddressingMode::AbsoluteX, 0x1E, 3, 7), 

        Opcode::new("BIT", AddressingMode::ZeroPage, 0x24, 2, 3),
        Opcode::new("BIT", AddressingMode::Absolute, 0x2C, 3, 4),

        Opcode::new("BCC", AddressingMode::Relative, 0x90, 2, 2), // + 1 if branch succeeds, + 2 if new page
        Opcode::new("BCS", AddressingMode::Relative, 0xB0, 2, 2), // + 1 if branch succeeds, + 2 if new page
        Opcode::new("BEQ", AddressingMode::Relative, 0xF0, 2, 2), // + 1 if branch succeeds, + 2 if new page
        Opcode::new("BMI", AddressingMode::Relative, 0x30, 2, 2), // + 1 if branch succeeds, + 2 if new page
        Opcode::new("BNE", AddressingMode::Relative, 0xD0, 2, 2), // + 1 if branch succeeds, + 2 if new page
        Opcode::new("BPL", AddressingMode::Relative, 0x10, 2, 2), // + 1 if branch succeeds, + 2 if new page
        Opcode::new("BVC", AddressingMode::Relative, 0x50, 2, 2), // + 1 if branch succeeds, + 2 if new page
        Opcode::new("BVS", AddressingMode::Relative, 0x70, 2, 2), // + 1 if branch succeeds, + 2 if new page

        Opcode::new("BRK", AddressingMode::Implied, 0x00, 1, 7),

        Opcode::new("CLC", AddressingMode::Implied, 0x18, 1, 2),

        Opcode::new("CLD", AddressingMode::Implied, 0xD8, 1, 2),

        Opcode::new("CLI", AddressingMode::Implied, 0x58, 1, 2),

        Opcode::new("CLV", AddressingMode::Implied, 0xB8, 1, 2),

        Opcode::new("CMP", AddressingMode::Immediate, 0xC9, 2, 2),
        Opcode::new("CMP", AddressingMode::ZeroPage, 0xC5, 2, 3),
        Opcode::new("CMP", AddressingMode::ZeroPageX, 0xD5, 2, 3),
        Opcode::new("CMP", AddressingMode::Absolute, 0xCD, 2, 4),
        Opcode::new("CMP", AddressingMode::AbsoluteX, 0xDD, 3, 4), // + 1
        Opcode::new("CMP", AddressingMode::AbsoluteY, 0xD9, 3, 4), // + 1
        Opcode::new("CMP", AddressingMode::IndexedIndirect, 0xC1, 2, 6),
        Opcode::new("CMP", AddressingMode::IndirectIndexed, 0xD1, 2, 5), // + 1

        Opcode::new("CPX", AddressingMode::Immediate, 0xE0, 2, 2),
        Opcode::new("CPX", AddressingMode::ZeroPage, 0xE4, 2, 3),
        Opcode::new("CPX", AddressingMode::Absolute, 0xEC, 3, 4),

        Opcode::new("CPY", AddressingMode::Immediate, 0xC0, 2, 2),
        Opcode::new("CPY", AddressingMode::ZeroPage, 0xC4, 2, 3),
        Opcode::new("CPY", AddressingMode::Absolute, 0xCC, 3, 4),

        Opcode::new("DEC", AddressingMode::ZeroPage, 0xC6, 2, 5),
        Opcode::new("DEC", AddressingMode::ZeroPageX, 0xD6, 2, 6),
        Opcode::new("DEC", AddressingMode::Absolute, 0xCE, 3, 6),
        Opcode::new("DEC", AddressingMode::AbsoluteX, 0xDE, 3, 7),

        Opcode::new("DEX", AddressingMode::Implied, 0xCA, 1, 2),

        Opcode::new("DEY", AddressingMode::Implied, 0x88, 1, 2),

        Opcode::new("EOR", AddressingMode::Immediate, 0x49, 2, 2),
        Opcode::new("EOR", AddressingMode::ZeroPage, 0x45, 2, 3),
        Opcode::new("EOR", AddressingMode::ZeroPageX, 0x55, 2, 4),
        Opcode::new("EOR", AddressingMode::Absolute, 0x4D, 3, 4),
        Opcode::new("EOR", AddressingMode::AbsoluteX, 0x5D, 3, 4), // + 1
        Opcode::new("EOR", AddressingMode::AbsoluteY, 0x59, 3, 4), // + 1
        Opcode::new("EOR", AddressingMode::IndexedIndirect, 0x41, 2, 6),
        Opcode::new("EOR", AddressingMode::IndirectIndexed, 0x51, 2, 5), // + 1

        Opcode::new("INC", AddressingMode::ZeroPage, 0xE6, 2, 5),
        Opcode::new("INC", AddressingMode::ZeroPageX, 0xF6, 2, 6),
        Opcode::new("INC", AddressingMode::Absolute, 0xEE, 3, 6),
        Opcode::new("INC", AddressingMode::AbsoluteX, 0xFE, 3, 7),

        Opcode::new("INX", AddressingMode::Implied, 0xE8, 1, 2),

        Opcode::new("INY", AddressingMode::Implied, 0xC8, 1, 2),

        Opcode::new("JMP", AddressingMode::Absolute, 0x4C, 3, 3),
        Opcode::new("JMP", AddressingMode::Indirect, 0x6C, 3, 5),

        Opcode::new("JSR", AddressingMode::Absolute, 0x20, 3, 6),

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

        Opcode::new("ORA", AddressingMode::Immediate, 0x09, 2, 2),
        Opcode::new("ORA", AddressingMode::ZeroPage, 0x05, 2, 3),
        Opcode::new("ORA", AddressingMode::ZeroPageX, 0x15, 2, 4),
        Opcode::new("ORA", AddressingMode::Absolute, 0x0D, 3, 4),
        Opcode::new("ORA", AddressingMode::AbsoluteX, 0x1D, 3, 4), // + 1
        Opcode::new("ORA", AddressingMode::AbsoluteY, 0x19, 3, 4), // + 1
        Opcode::new("ORA", AddressingMode::IndexedIndirect, 0x01, 2, 6),
        Opcode::new("ORA", AddressingMode::IndirectIndexed, 0x11, 2, 5), // + 1

        Opcode::new("PHA", AddressingMode::Implied, 0x48, 1, 3), 
        Opcode::new("PHP", AddressingMode::Implied, 0x08, 1, 3), 
        Opcode::new("PLA", AddressingMode::Implied, 0x68, 1, 4), 
        Opcode::new("PLP", AddressingMode::Implied, 0x28, 1, 4), 

        Opcode::new("ROL", AddressingMode::Accumulator, 0x2A, 1, 2),
        Opcode::new("ROL", AddressingMode::ZeroPage, 0x26, 2, 5),
        Opcode::new("ROL", AddressingMode::ZeroPageX, 0x36, 2, 6),
        Opcode::new("ROL", AddressingMode::Absolute, 0x2E, 3, 6),
        Opcode::new("ROL", AddressingMode::AbsoluteX, 0x3E, 3, 7),

        Opcode::new("ROR", AddressingMode::Accumulator, 0x6A, 1, 2),
        Opcode::new("ROR", AddressingMode::ZeroPage, 0x66, 2, 5),
        Opcode::new("ROR", AddressingMode::ZeroPageX, 0x76, 2, 6),
        Opcode::new("ROR", AddressingMode::Absolute, 0x6E, 3, 6),
        Opcode::new("ROR", AddressingMode::AbsoluteX, 0x7E, 3, 7),

        Opcode::new("RTI", AddressingMode::Implied, 0x40, 1, 6),

        Opcode::new("RTS", AddressingMode::Implied, 0x60, 1, 6),

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