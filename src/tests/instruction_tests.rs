#[cfg(test)]
mod tests {
    use crate::chip8::cpu::{Cpu, PROGRAM_COUNTER_START_ADDR};
    use crate::modules::display::BYTES_PER_CHARACTER;

    #[test]
    fn test_ret() {
        let mut cpu = init(vec!(0x00, 0xEE));
        cpu.push(0xFF);
        cpu.run();

        assert_eq!(cpu.program_counter, 0xFF);
    }

    #[test]
    fn test_jmp() {
        let mut cpu = init(vec!(0x10, 0xFF));
        cpu.run();

        assert_eq!(cpu.program_counter, 0x0FF);
    }

    #[test]
    fn test_call() {
        let mut cpu = init(vec!(0x2F, 0xFF));
        let pc = cpu.program_counter + 2;
        cpu.run();
        assert_eq!(cpu.program_counter, 0xFFF);
        let stack = cpu.pop();

        assert_eq!(stack, pc);
    }

    #[test]
    fn test_se_vx_kk() {
        // SE V0 kk
        let mut cpu = init(vec!(0x30, 12));
        cpu.registers[0] = 12;
        cpu.run();

        assert_eq!(cpu.program_counter, PROGRAM_COUNTER_START_ADDR + 4);
    }

    #[test]
    fn test_sne_vx_kk() {
        // SNE V0 kk
        let mut cpu = init(vec!(0x40, 12));
        cpu.run();

        assert_eq!(cpu.program_counter, PROGRAM_COUNTER_START_ADDR + 4);
    }

    #[test]
    fn test_se_vx_vy() {
        // SE V0, V1
        let mut cpu = init(vec!(0x50, 0x10));
        cpu.registers[0] = 1;
        cpu.registers[1] = 1;
        cpu.run();

        assert_eq!(cpu.program_counter, PROGRAM_COUNTER_START_ADDR + 4);
    }

    #[test]
    fn test_ld_vx_kk() {
        let mut cpu = init(vec!(0x60, 0x12));
        cpu.run();

        assert_eq!(cpu.registers[0], 0x12);
    }

    #[test]
    fn test_add_vx_kk() {
        let reg: usize = 0;
        let mut cpu = init(vec!(0x70, 0x12));
        cpu.registers[reg] = 0x05;
        cpu.run();

        assert_eq!(cpu.registers[reg], 0x12 + 0x05);
    }

    #[test]
    fn test_ld_vx_vy() {
        let mut cpu = init(vec!(0x80, 0x10));

        cpu.registers[1] = 0xFF;
        cpu.run();

        assert_eq!(cpu.registers[0], 0xFF);
    }

    #[test]
    fn test_or_vx_vy() {
        let mut cpu = init(vec!(0x80, 0x11));
        cpu.registers[0] = 0xF0;
        cpu.registers[1] = 0x0F;

        cpu.run();

        assert_eq!(cpu.registers[0], 0xFF);
    }

    #[test]
    fn test_and_vx_vy() {
        let mut cpu = init(vec!(0x80, 0x12));
        cpu.registers[0] = 0xF0;
        cpu.registers[1] = 0xFF;
        cpu.run();

        assert_eq!(cpu.registers[0], 0xF0);
    }

    #[test]
    fn test_xor_vx_vy() {
        let mut cpu = init(vec!(0x80, 0x13));
        cpu.registers[0] = 0xF0;
        cpu.registers[1] = 0x00;
        cpu.run();

        assert_eq!(cpu.registers[0], 0xF0);
    }

    #[test]
    fn test_add_vx_vy() {
        let mut cpu = init(vec!(0x80, 0x14));
        cpu.registers[0] = 5;
        cpu.registers[1] = 10;

        cpu.run();

        assert_eq!(cpu.registers[0], 15);
        assert_eq!(cpu.registers[0x0F], 0);
    }

    #[test]
    fn test_add_vx_vy_vf() {
        let mut cpu = init(vec!(0x80, 0x14));
        cpu.registers[0] = 255;
        cpu.registers[1] = 2;

        cpu.run();

        assert_eq!(cpu.registers[0], 1);
        assert_eq!(cpu.registers[0x0F], 1);
    }

    #[test]
    fn test_sub_vx_vy() {
        let mut cpu = init(vec!(0x80, 0x15));
        cpu.registers[0] = 15;
        cpu.registers[1] = 10;

        cpu.run();

        assert_eq!(cpu.registers[0], 5);
        assert_eq!(cpu.registers[0x0F], 1);
    }

    #[test]
    fn test_sub_vx_vy_vf() {
        let mut cpu = init(vec!(0x80, 0x15));
        cpu.registers[0] = 1;
        cpu.registers[1] = 2;
        cpu.run();

        assert_eq!(cpu.registers[0], 255);
        assert_eq!(cpu.registers[0x0F], 0);
    }

    #[test]
    fn test_shr_vx() {
        let mut cpu = init(vec!(0x80, 0x06));
        cpu.registers[0] = 1;
        cpu.run();

        assert_eq!(cpu.registers[0], 1 >> 1);
    }

    #[test]
    fn test_shr_vx_vf() {
        let mut cpu = init(vec!(0x80, 0x06));
        cpu.registers[0] = 0b00001111;
        cpu.run();

        assert_eq!(cpu.registers[0], 0b00001111 >> 1);
        assert_eq!(cpu.registers[0x0F], 1);
    }

    #[test]
    fn test_subn_vx_vy() {
        let mut cpu = init(vec!(0x80, 0x17));
        cpu.registers[0] = 10;
        cpu.registers[1] = 15;
        cpu.run();

        assert_eq!(cpu.registers[0], 5);
        assert_eq!(cpu.registers[0x0F], 1);
    }

    #[test]
    fn test_subn_vx_vy_vf() {
        let mut cpu = init(vec!(0x80, 0x17));
        cpu.registers[0] = 2;
        cpu.registers[1] = 1;
        cpu.run();

        assert_eq!(cpu.registers[0], 255);
        assert_eq!(cpu.registers[0x0F], 0);
    }

    #[test]
    fn test_shl_vx() {
        let mut cpu = init(vec!(0x80, 0x0E));
        cpu.registers[0] = 2;
        cpu.run();

        assert_eq!(cpu.registers[0], 4);
        assert_eq!(cpu.registers[0x0F], 0);
    }

    #[test]
    fn test_shl_vx_vf() {
        let mut cpu = init(vec!(0x80, 0x0E));
        cpu.registers[0] = 0xF0;
        cpu.run();

        assert_eq!(cpu.registers[0], 0xF0 << 1);
        assert_eq!(cpu.registers[0x0F], 1);
    }

    #[test]
    fn test_sne_vx_vy() {
        let mut cpu = init(vec!(0x90, 0x10));
        cpu.registers[0] = 1;
        cpu.run();

        assert_eq!(cpu.program_counter, PROGRAM_COUNTER_START_ADDR + 4);
    }

    #[test]
    fn test_ld_i_addr() {
        let mut cpu = init(vec!(0xAF, 0xFF));
        cpu.run();

        assert_eq!(cpu.i, 0xFFF);
    }

    #[test]
    fn test_jp_v0() {
        let mut cpu = init(vec!(0xBF, 0xFF));
        cpu.registers[0] = 1;
        cpu.run();

        assert_eq!(cpu.program_counter, 0xFFF + 1);
    }

    #[test]
    fn test_ld_vx_dt() {
        let mut cpu = init(vec!(0xF0, 0x07));
        cpu.delay_timer = 10;
        cpu.run();

        assert_eq!(cpu.registers[0], 10);
    }

    #[test]
    fn test_ld_dt_vx() {
        let mut cpu = init(vec!(0xF0, 0x15));
        cpu.registers[0] = 10;
        cpu.run();

        assert_eq!(cpu.delay_timer, 10 - 1);
    }

    #[test]
    fn test_ld_st_vx() {
        let mut cpu = init(vec!(0xF0, 0x18));
        cpu.registers[0] = 10;
        cpu.run();

        assert_eq!(cpu.sound_timer, 10 - 1);
    }

    #[test]
    fn test_add_i_vx() {
        let mut cpu = init(vec!(0xF0, 0x1E));
        cpu.registers[0] = 10;
        cpu.i = 10;
        cpu.run();

        assert_eq!(cpu.i, 20);
    }

    #[test]
    fn test_ld_f_vx() {
        let mut cpu = init(vec!(0xF0, 0x29));
        cpu.registers[0] = 10;
        cpu.run();

        assert_eq!(cpu.i, 10 * BYTES_PER_CHARACTER as usize);
    }

    fn init(program: Vec<u8>) -> Cpu {
        Cpu::new(&program)
    }
}