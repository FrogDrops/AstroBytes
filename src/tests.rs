#[allow(unused_imports)]
use crate::processor::*;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_transfer_ops() {
        let mut cpu = CPU::new();
        cpu.load_and_execute(vec![0xA9, 0x0A, 0xAA, 0x00]);
        assert_eq!(cpu.register_x, 10); 

        cpu.load_and_execute(vec![0xA9, 0x0A, 0xA8, 0x00]);
        assert_eq!(cpu.register_y, 10);  

        cpu.load_and_execute(vec![0xBA, 0x00]);
        assert_eq!(cpu.register_x, 0xFF);  

        cpu.load_and_execute(vec![0xA2, 0x0A, 0x8A, 0x00]);
        assert_eq!(cpu.register_a, 10);

        cpu.load_and_execute(vec![0xA2, 0x0A, 0x9A, 0x00]);
        assert_eq!(cpu.stack_pointer, 10);

        cpu.load_and_execute(vec![0xA0, 0x0A, 0x98, 0x00]);
        assert_eq!(cpu.register_a, 10);
    }

    #[test]
    fn test_five_ops() {
        let mut cpu = CPU::new();
        cpu.load_and_execute(vec![0xA9, 0xC0, 0xAA, 0xE8, 0x00]);
        assert_eq!(cpu.register_x, 0xC1)
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.load_and_execute(vec![0xA2, 0xFF, 0xE8, 0xE8, 0x00]);
        assert_eq!(cpu.register_x, 1)
    }

    #[test]
    fn test_lda() {
        let mut cpu = CPU::new();

        // Immediate
        cpu.load_and_execute(vec![0xA9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert!(cpu.status_flags & 0b0000_0010 == 0b00);
        assert!(cpu.status_flags & 0b1000_0000 == 0);

        // Zero page
        cpu.load_and_execute(vec![0xA9, 0xA0, 0xA5, 0xFF, 0x00]);
        assert_eq!(cpu.register_a, 0);

        // Absolute
        cpu.load_and_execute(vec![0xA9, 0x0A, 0x8D, 0xFF, 0x01, 0xA9, 0xAA, 0xAD, 0xFF, 0x01, 0x00]);
        assert_eq!(cpu.register_a, 0x0A);

        // Zero flag
        cpu.load_and_execute(vec![0xA9, 0x00, 0x00]);
        assert!(cpu.status_flags & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_ldx() {
        let mut cpu = CPU::new();

        // Immediate
        cpu.load_and_execute(vec![0xA2, 0x05, 0x00]);
        assert_eq!(cpu.register_x, 0x05);
        assert!(cpu.status_flags & 0b0000_0010 == 0b00);
        assert!(cpu.status_flags & 0b1000_0000 == 0);
 
        // Zero page + y
        cpu.load_and_execute(vec![0xA0, 0x01, 0xA9, 0x02, 0x85, 0x08, 0xB6, 0x07, 0x00]);
        assert_eq!(cpu.register_x, 0x02);
 
        // Absolute
        cpu.load_and_execute(vec![0xA9, 0x0A, 0x8D, 0xFF, 0x01, 0xAE, 0xFF, 0x01, 0x00]);
        assert_eq!(cpu.register_x, 0x0A);
 
        // Zero flag
        cpu.load_and_execute(vec![0xA2, 0x00, 0x00]);
        assert!(cpu.status_flags & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_ldy() {
        let mut cpu = CPU::new();

        // Immediate
        cpu.load_and_execute(vec![0xA2, 0x05, 0x00]);
        assert_eq!(cpu.register_x, 0x05);
        assert!(cpu.status_flags & 0b0000_0010 == 0b00);
        assert!(cpu.status_flags & 0b1000_0000 == 0);
 
        // Zero page
        cpu.load_and_execute(vec![0xA0, 0xA0, 0xA4, 0xFF, 0x00]);
        assert_eq!(cpu.register_y, 0);
 
        // Absolute
        cpu.load_and_execute(vec![0xA9, 0x0A, 0x8D, 0xFF, 0x01, 0xAC, 0xFF, 0x01, 0x00]);
        assert_eq!(cpu.register_y, 0x0A);
 
        // Zero flag
        cpu.load_and_execute(vec![0xA0, 0x00, 0x00]);
        assert!(cpu.status_flags & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_sta_stx_sty() {
        let mut cpu: CPU = CPU::new();
        cpu.load_and_execute(vec![0xA9, 0x05, 0x85, 0x30, 0x00]);
        let result = cpu.read_memory_u8(0x30);
        assert_eq!(result, 0x05);

        cpu.load_and_execute(vec![0xA2, 0x05, 0x86, 0x30, 0x00]);
        let result = cpu.read_memory_u8(0x30);
        assert_eq!(result, 0x05);

        cpu.load_and_execute(vec![0xA0, 0x05, 0x84, 0x30, 0x00]);
        let result = cpu.read_memory_u8(0x30);
        assert_eq!(result, 0x05);
    }

    #[test]
    fn test_sei_sed_sec() {
        let mut cpu: CPU = CPU::new();
        cpu.load_and_execute(vec![0x78, 0x00]);
        assert_eq!(cpu.status_flags, 0b0000_0100);

        cpu.load_and_execute(vec![0xF8, 0x00]);
        assert_eq!(cpu.status_flags, 0b0000_1000);

        cpu.load_and_execute(vec![0x38, 0x00]);
        assert_eq!(cpu.status_flags, 0b0000_0001);
    }
}
