/*
   For the CPU component (also known as the 2A03 chip in the case of the NES :D):
   Fetch next execution instruction from the instruction memory
   Decode the instruction
   Execute the Instruction
   Repeat the cycle

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

pub mod processor {
    pub struct CPU {
        pub register_a: u8,
        pub register_x: u8,
        pub register_y: u8,
        pub status: u8,
        pub program_counter: u16,
        // 64 kiB of address space
        memory: [u8; 0xFFFF]
    }

    impl CPU {
        pub fn new() -> Self {
            CPU {
                register_a: 0,
                register_x: 0,
                register_y: 0,
                status: 0,
                program_counter: 0,
                memory: [0; 0xFFFF]
            }
        }

        pub fn run_program(&mut self) {
            loop {
                let opcode = self.read_memory(self.program_counter);
                self.program_counter += 1;

                match opcode {
                    // BRK opcode
                    0x00 => return,

                    // Takes two bytes of memory (one for the opcode, another for its argument)
                    0xA9 => {
                        let argument = program[self.program_counter as usize];
                        self.program_counter += 1;
                        self.lda(argument);
                    }

                    0xAA => self.tax(),

                    0xE8 => self.inx(),

                    _ => todo!(),
                }
            }
        }

        fn read_memory(&self, address: u16) -> u8 {
            self.memory[address as usize]
        }

        fn write_memory(&mut self, address: u16, data: u8) {
            self.memory[address as usize] = data;
        }

        /*
        An address is 2 bytes long.
        
        When reading memory,
        we need to reverse the little-endian addressing of the NES
        (The LSB is stored first followed by the MSB).
        We thus reorder the bits (MSB to LSB) and 
        combine them into one u16 bit value

        Following this convention, when writing memory,
        we write the LSB first followed by the MSB
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

        // Load program into program ROM space [0x8000.. 0xFFFF] 
        pub fn load_program(&mut self, program: Vec<u8>) {
            self.memory[0x8000..(0x8000 + program.len())].copy_from_slice(&program[..]);
            self.write_memory_u16(0xFFFC, 0x8000);
        }

        pub fn load_and_run_program(&mut self, program: Vec<u8>) {
            self.load_program(program);
            self.run_program();
        }

        pub fn reset(&mut self) {
            self.register_a = 0;
            self.register_x = 0;
            self.register_y = 0;
            self.status = 0;
            self.program_counter = self.read_memory_u16(0xFFFC);
        }

        fn lda(&mut self, value: u8) {
            self.register_a = value;
            self.zero_and_negative_flags(self.register_a);
        }

        fn tax(&mut self) {
            self.register_x = self.register_a;
            self.zero_and_negative_flags(self.register_x);
        }

        fn inx(&mut self) {
            self.register_x = self.register_x.wrapping_add(1);
            self.zero_and_negative_flags(self.register_x);

            if self.register_x == 0 {
                self.set_overflow_flag();
            } else {
                self.clear_overflow_flag();
            }
        }

        fn set_overflow_flag(&mut self) {
            self.status = self.status | 0b0100_0000;
        }

        fn clear_overflow_flag(&mut self) {
            self.status = self.status & 0b1011_1111;
        }

        fn zero_and_negative_flags(&mut self, result: u8) {
            // Zero flag
            if result == 0 {
                self.status = self.status | 0b0000_0010;
            } else {
                self.status = self.status & 0b1111_1101;
            }

            // Negative flag
            if self.register_x & 0b1000_0000 != 0 {
                self.status = self.status | 0b1000_0000;
            } else {
                self.status = self.status & 0b0111_1111;
            }
        }
    }
}
