#[allow(unused_imports)]
use crate::processor::processor::*;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lda_immediate_load() {
        let mut cpu = CPU::new();
        cpu.run_program();
        assert_eq!(cpu.register_a, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0b00);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.run_program();
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_tax() {
        let mut cpu = CPU::new();
        cpu.register_a = 10;
        cpu.run_program();

        assert_eq!(cpu.register_x, 10)
    }

    #[test]
    fn test_five_ops() {
        let mut cpu = CPU::new();
        cpu.run_program();

        assert_eq!(cpu.register_x, 0xc1)
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.register_x = 0xff;
        cpu.run_program();

        assert_eq!(cpu.register_x, 1)
    }
}
