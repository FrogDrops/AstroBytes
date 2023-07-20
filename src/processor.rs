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

            match opcode {

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
        self.register_x += 1;
        self.zero_and_negative_flags(self.register_x);
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


#[cfg(test)]
mod test 
{
   use super::*;
 
   #[test]
   fn test_lda_immediate_load_data() 
   {
       let mut cpu = CPU::new();
       cpu.interpret(vec![0xa9, 0x05, 0x00]);
       assert_eq!(cpu.register_a, 0x05);
       assert!(cpu.status & 0b0000_0010 == 0b00);
       assert!(cpu.status & 0b1000_0000 == 0);
   }

    #[test]
    fn test_lda_zero_flag() 
    {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x00, 0x00]);
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }

    #[test]
   fn test_tax() 
   {
       let mut cpu = CPU::new();
       cpu.register_a = 10;
       cpu.interpret(vec![0xaa, 0x00]);
 
       assert_eq!(cpu.register_x, 10)
   }

   #[test]
   fn test_five_ops() 
   {
       let mut cpu = CPU::new();
       cpu.interpret(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);
 
       assert_eq!(cpu.register_x, 0xc1)
   }

    #[test]
    fn test_inx_overflow() 
    {
        let mut cpu = CPU::new();
        cpu.register_x = 0xff;
        cpu.interpret(vec![0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 1)
    }
}