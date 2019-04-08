use core::fmt;
use rand::Rng;

const MEMORY_SIZE: u16 = 4096;

pub struct Cpu {
    // memory
    memory: [u8; MEMORY_SIZE as usize],
    // registers (V0-VF)
    registers: [u8; 16],
    i: u16, // address register

    program_counter: u16,

    // stack
    stack: [u16; 16],
    stack_pointer: u8,

    // timers
    delay_timer: u8,
    sound_timer: u8,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            memory: [0; MEMORY_SIZE as usize],
            registers: [0; 16],
            i: 0,
            program_counter: 0,
            stack: [0; 16],
            stack_pointer: 0,
            delay_timer: 0,
            sound_timer: 0
        }
    }

    fn read(&self, address: u16) -> u8 {
        if address > self.memory.len() as u16 {
            panic!("Read at {:x} out of bounds", address);
        }

        self.memory[address as usize]
    }

    fn read_word(&self, address: u16) -> u16 {
        let low = self.read(address);
        let high = self.read(address + 1);

        ((high as u16) << 8) | (low as u16)
    }

    fn read_word_increment_pc(&mut self, address: u16) -> u16 {
        let word = self.read_word(address);
        self.program_counter += 2;
        word
    }

    fn write(&mut self, address: u16, value: u8) {
        if address > self.memory.len() as u16 {
            panic!("write at {:x} out of bounds", address);
        }

        self.memory[address as usize] = value;
    }

    fn pop(&mut self) -> u16 {
        let val = self.stack[self.stack_pointer as usize];
        self.stack_pointer -= 1;
        val
    }

    fn push(&mut self, value: u16) {
        self.stack[self.stack_pointer as usize] = value;
        self.stack_pointer += 1;
    }

    fn get_register_value(&self, index: u16) -> u8 {
        self.registers[index as usize]
    }

    fn emulate_cycle(&mut self) {
        let pc = self.program_counter;

        // the full 16 bytes of an instruction, including operands
        let instruction = self.read_word(pc);
        self.program_counter += 2;

        // opcodes are stored in the first 2 bytes of an instruction, big endian
        let opcode = instruction & 0xF000;
        match opcode {
            // 0nnn - SYS addr
            // Jump to routine at nnn
            0x00 => {
                // not used in interpreters
                panic!("unsupported SYS routine");
            },
            // 00E0 - CLS
            // Clear display
            0xE0 => {

            },
            // 00EE - RET
            // Return from a subroutine i.e. set pc to top of stack
            0xEE => {
                self.program_counter = self.pop();
            }
            // 1nnn - JMP addr
            // Jump to address nnn
            0x01 => {
                let addr = instruction & 0x0FFF;
                self.program_counter = addr;
            }
            // 2nnn - CALL addr
            // Call subroutine at nnn
            0x02 => {
                let addr = instruction & 0x0FFF;
                let pc = self.program_counter;
                self.push(pc);
                self.program_counter = addr;
            }
            // 3xkk - SE Vx, byte
            // Skip next instruction if Vx == kk
            // 4xkk - SNE Vx, byte
            // Skip next instruction if Vx != kk
            0x03 | 0x04 => {
                let register = instruction & 0x0F00 >> 8;
                let register = self.get_register_value(register) as u16;
                let cmp = instruction & 0x00FF;
                if opcode == 0x03 && register == cmp || opcode == 0x04 && register != cmp {
                    self.program_counter += 2;
                }
            },
            // 5xy0 - SE Vx, Vy
            // Skip next instruction if Vx = Vy
            0x05 => {
                let register_x = instruction & 0x0F00 >> 8;
                let register_y = instruction & 0x00F0 >> 4;
                let register_x = self.get_register_value(register_x);
                let register_y = self.get_register_value(register_y);

                if register_x == register_y {
                    self.program_counter += 2;
                }
            },
            // 6xkk - LD Vx, byte
            // Set Vx to value kk
            0x06 => {
                let register = instruction & 0x0F00 >> 8;
                let value = (instruction & 0x00FF) as u8;
                self.registers[register as usize] = value;
            },
            // 7xkk ADD Vx, byte
            // Set Vx = Vx + kk
            0x07 => {
                let register = instruction & 0x0F00 >> 8;
                let value = (instruction & 0x00FF) as u8;
                self.registers[register as usize] += value;
            },
            0x08 => {
                // 8xy0 - LD Vx, Vy
                // Set Vx to value of Vy

                // 8xy1 - OR Vx, Vy
                // Set Vx = Vx OR Vy.

                // 8xy2 - AND Vx, Vy
                // Set Vx = Vx AND Vy.

                // 8xy3 - XOR Vx, Vy
                // Set Vx = Vx XOR Vy.

                // 8xy4 - ADD Vx, Vy
                // Set Vx = Vx + Vy, set VF = carry.

                // 8xy5 - SUB Vx, Vy
                // Set Vx = Vx - Vy, set VF = NOT borrow.

                // 8xy6 - SHR Vx {, Vy}
                // Set Vx = Vx SHR 1.

                // 8xy7 - SUBN Vx, Vy
                // Set Vx = Vy - Vx, set VF = NOT borrow.

                // 8xyE - SHL Vx {, Vy}
                // Set Vx = Vx SHL 1.
            }

            // 9xy0 - SNE Vx, Vy
            // Skip next instruction if Vx != Vy
            0x09 => {

            },

            // Annn - LD I, addr
            // Set register I to nnn
            0x0A => {
                let value = instruction & 0x0FFF;
                self.i = value;
            }

            // Bnnn - JP V0, addr
            // Jump to location nnn + V0
            0x0B => {
                let v0 = self.registers[0] as u16;
                let address = instruction & 0x0FFF;
                self.program_counter = v0 + address;
            },
            // Cxkk - RND Vx, byte
            // Set Vx = random byte (0-255) AND kk
            0x0C => {
                let register = instruction & 0x0F00 >> 8;
                let rng = rand::thread_rng().gen_range(0, 256) as u8;
                let value = (instruction & 0x00FF) as u8;

                self.registers[register as usize] = rng & value;
            },
            // Dxyn - DRW Vx, Vy, nibble
            // Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision
            0x0D => {

            },
            0x0E => {
                // Ex9E - SKP Vx
                // Skip next instruction if key with the value of Vx is pressed.
            },
            0x0F => {

            },
            _ => panic!("Unrecognized opcode {:x} in instruction {:x}", opcode, instruction)
        }
    }
}

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TODO cpu fmt")
    }
}