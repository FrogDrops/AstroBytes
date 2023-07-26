#[allow(unused_imports)]
use crate::processor::*;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lda_immediate_load() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert!(cpu.status_flags & 0b0000_0010 == 0b00);
        assert!(cpu.status_flags & 0b1000_0000 == 0);
    }

    #[test]
    fn test_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x00, 0x00]);
        assert!(cpu.status_flags & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_tax() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x0A, 0xAA, 0x00]);

        assert_eq!(cpu.register_x, 10);     
    }

    #[test]
    fn test_five_ops() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0xC0, 0xAA, 0xE8, 0x00]);

        assert_eq!(cpu.register_x, 0xC1)
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.register_x = 0xFF;
        cpu.load_and_run(vec![0xE8, 0xE8, 0x00]);

        assert_eq!(cpu.register_x, 1)
    }
}
