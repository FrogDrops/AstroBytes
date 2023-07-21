/*
    Abbreviations: 

    For the CPU component (also known as the 2A03 chip in the case of the NES :D):
    Fetch next execution instruction from the instruction memory
    Decode the instruction
    Execute the Instruction
    Repeat the cycle

    C: Carry Flag (LSB)
    Z: Zero Flag
    I: Interrupt Disable
    D: Decimal Mode Flag
    The B Flag
    B: Break Command
    V: Overflow Flag
    N: Negative Flag (MSB)
 */

pub mod processor
{
    pub struct CPU 
    {
        pub register_a: u8,
        pub register_x: u8,
        pub register_y: u8,
        pub status: u8,
        pub program_counter: u16
    }

    impl CPU
    {
        pub fn new() -> Self
        {
            CPU 
            {
                register_a: 0,
                register_x: 0,
                register_y: 0,
                status: 0,
                program_counter: 0
            }
        }

        pub fn interpret(&mut self, program: Vec<u8>)
        {
            self.program_counter = 0;

            loop
            {
                let opcode = program[self.program_counter as usize];
                self.program_counter += 1;

                match opcode 
                {
                    // BRK opcode
                    0x00 => return,

                    // Takes two bytes of memory (one for the opcode, another for its argument)
                    0xA9 => 
                    {
                        let argument = program[self.program_counter as usize];
                        self.program_counter += 1;
                        self.lda(argument);
                    }

                    0xAA => self.tax(),

                    0xE8 => self.inx(),

                    _ => todo!()
                }
            }
        }

        fn lda(&mut self, value: u8)
        {
            self.register_a = value;
            self.zero_and_negative_flags(self.register_a);
        }

        fn tax(&mut self)
        {
            self.register_x = self.register_a;
            self.zero_and_negative_flags(self.register_x);
        }

        fn inx(&mut self)
        {
            self.register_x = self.register_x.wrapping_add(1);
            self.zero_and_negative_flags(self.register_x);

            if self.register_x == 0 {
                self.set_overflow_flag();
            }
            else {
                self.clear_overflow_flag();
            }
        }

        fn set_overflow_flag(&mut self)
        {
            self.status = self.status | 0b0100_0000;
        }

        fn clear_overflow_flag(&mut self)
        {
            self.status = self.status & 0b1011_1111;
        }

        fn zero_and_negative_flags(&mut self, result: u8)
        {
            // Zero flag
            if result == 0 {
                self.status = self.status | 0b0000_0010;
            }
            else {
                self.status = self.status & 0b1111_1101;
            }
        
            // Negative flag
            if self.register_x & 0b1000_0000 != 0 {
                self.status = self.status | 0b1000_0000;
            }
            else {
                self.status = self.status & 0b0111_1111;
            }
        }
    }
}

