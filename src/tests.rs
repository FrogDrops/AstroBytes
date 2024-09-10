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
    fn test_dec_dex_dey() {
        let mut cpu: CPU = CPU::new();

        cpu.load_and_execute(vec![0xA2, 0x09, 0xCA, 0x00]);
        assert_eq!(cpu.register_x, 8);

        cpu.load_and_execute(vec![0xA0, 0x03, 0x88, 0x00]);
        assert_eq!(cpu.register_y, 2);

        cpu.load_and_execute(vec![0xA9, 0x09, 0x85, 0x05, 0xC6, 0x05, 0x00]);
        let result = cpu.read_memory_u8(0x05);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_inc_inx_iny() {
        let mut cpu = CPU::new();

        // Tests INX overflow
        cpu.load_and_execute(vec![0xA2, 0xFF, 0xE8, 0xE8, 0x00]);
        assert_eq!(cpu.register_x, 1);

        cpu.load_and_execute(vec![0xA0, 0x03, 0xC8, 0x00]);
        assert_eq!(cpu.register_y, 4);

        cpu.load_and_execute(vec![0xE6, 0x05, 0x00]);
        let result = cpu.read_memory_u8(0x05);
        assert_eq!(result, 1);
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

    #[test]
    fn test_asl() {
        let mut cpu: CPU = CPU::new();
        cpu.load_and_execute(vec![0xA9, 0x02, 0x0A, 0x00]);
        assert_eq!(cpu.register_a, 4);

        cpu.load_and_execute(vec![0xA9, 0x02, 0x85, 0x07, 0x06, 0x07, 0x00]);
        let result = cpu.read_memory_u8(0x07);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_and_or_xor() {
        let mut cpu: CPU = CPU::new();
        cpu.load_and_execute(vec![0xA2, 0x06, 0x86, 0x07, 0xA9, 0x04, 0x25, 0x07, 0x00]);
        assert_eq!(cpu.register_a, 4);

        cpu.load_and_execute(vec![0xA9, 0x06, 0x09, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 7);

        cpu.load_and_execute(vec![0xA9, 0x06, 0x49, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 3); 
    }

    // Reminder that the program is loaded into 0x600! 
    #[test]
    fn test_jmp_jsr_rts() {
        let mut cpu: CPU = CPU::new();
        cpu.load_and_execute(vec![0xA9, 0x0A, 0x20, 0x06, 0x06, 0x00, 0xA9, 0x01, 0x60, 0x00]);
        assert_eq!(cpu.register_a, 1); 

        cpu.load_and_execute(vec![0xA2, 0x0A, 0x4C, 0x07, 0x06, 0xA2, 0x05, 0xE8, 0x00]);
        assert_eq!(cpu.register_x, 11); 
    }

    #[test]
    fn test_pha_pla_php_plp() {
        let mut cpu: CPU = CPU::new();
        cpu.load_and_execute(vec![0xA9, 0x42, 0x48, 0x68, 0x00]);
        assert_eq!(cpu.register_a, 0x42);

        cpu.load_and_execute(vec![0x08, 0x38, 0x28, 0x00]);
        assert_eq!(cpu.status_flags, 0b0000_0000);
    }

    #[test]
    fn test_rol_ror() {
        let mut cpu: CPU = CPU::new();
        cpu.load_and_execute(vec![0xA9, 0x85, 0x2A, 0x00]);
        assert_eq!(cpu.register_a, 0x0A);

        cpu.load_and_execute(vec![0xA9, 0x12, 0x2A, 0x00]);
        assert_eq!(cpu.register_a, 0x24);

        cpu.load_and_execute(vec![0xA9, 0x04, 0x6A, 0x00]);
        assert_eq!(cpu.register_a, 0x02);

        cpu.load_and_execute(vec![0xA9, 0x06, 0x6A, 0x00]);
        assert_eq!(cpu.register_a, 0x02);
    }

    #[test]
    fn test_adc_sbc() {
        let mut cpu: CPU = CPU::new();
        cpu.load_and_execute(vec![0xA9, 0x05, 0x69, 0x0A, 0x00]);
        assert_eq!(cpu.register_a, 0x0F);

        cpu.load_and_execute(vec![0xA9, 0x81, 0x38, 0x69, 0x7F, 0x00]);
        assert_eq!(cpu.register_a, 0x01);

        cpu.load_and_execute(vec![0xA9, 0x0A, 0xE9, 0x03, 0x00]); // 10 - 3
        assert_eq!(cpu.register_a, 0x07);

        cpu.load_and_execute(vec![0xA9, 0x05, 0x38, 0xE9, 0x0A, 0x00]); // 5 - 10 (with carry)
        assert_eq!(cpu.register_a, 0xFC); 

        cpu.load_and_execute(vec![0xA9, 0x05, 0xE9, 0x0A, 0x00]); // 5 - 10 (without carry)
        assert_eq!(cpu.register_a, 0xFB); 

        cpu.load_and_execute(vec![0xA9, 0x0A, 0x38, 0xE9, 0x05, 0x00]); // 10 - 5 (with carry)
        assert_eq!(cpu.register_a, 0x06); 

        cpu.load_and_execute(vec![0xA9, 0x0A, 0xE9, 0x05, 0x00]); // 10 - 5 without carry
        assert_eq!(cpu.register_a, 0x05); 
    }

    #[test]
    fn test_branch() {
        let mut cpu: CPU = CPU::new();
        cpu.load_and_execute(vec![0x38, 0xB0, 0x02, 0xA9, 0x0A, 0xA9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 0x05);

        cpu.load_and_execute(vec![0x10, 0x02, 0xA9, 0x0A, 0xA9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
    }
}
