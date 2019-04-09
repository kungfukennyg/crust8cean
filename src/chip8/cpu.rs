use core::fmt;
use rand::Rng;
use std::num::Wrapping;

const MEMORY_SIZE: u16 = 4096;
const SCREEN_WIDTH: u8 = 64;
const SCREEN_HEIGHT: u8 = 32;
const SCREEN_SIZE: u16 = SCREEN_WIDTH as u16 * SCREEN_HEIGHT as u16;

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

    // screen
    screen: [u8; SCREEN_SIZE as usize]
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
            sound_timer: 0,
            screen: [0; SCREEN_SIZE as usize]
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

    fn set_carry_flag(&mut self, value: u8) {
        self.registers[0x0F] = value;
    }

    fn write_to_screen(&mut self, x: u8, y:u8, values: Vec<u8>) {
        // TODO wrap around
        let pos = x * SCREEN_WIDTH + y / SCREEN_HEIGHT;
        let prev = self.screen[pos as usize];
        for (_i, val) in values.iter().enumerate() {
            let cur = self.screen[pos as usize];
            self.screen[pos as usize] = cur ^ val;
        }

        if prev != self.screen[pos as usize] {
            self.set_carry_flag(1);
        } else {
            self.set_carry_flag(0);
        }
    }

    fn emulate_cycle(&mut self) {
        let pc = self.program_counter;

        // the full 16 bits of an instruction, including operands
        let instruction = self.read_word(pc);
        self.program_counter += 2;

        // opcodes are stored in the first 2 bits of an instruction, big endian
        let opcode = instruction & 0xF000 >> 6;
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
                let register = instruction & 0x0F00 >> 4;
                let register = self.get_register_value(register) as u16;
                let cmp = instruction & 0x00FF;
                if opcode == 0x03 && register == cmp || opcode == 0x04 && register != cmp {
                    self.program_counter += 2;
                }
            },
            // 5xy0 - SE Vx, Vy
            // Skip next instruction if Vx = Vy
            0x05 => {
                let register_x = instruction & 0x0F00 >> 4;
                let register_y = instruction & 0x00F0 >> 2;
                let register_x = self.get_register_value(register_x);
                let register_y = self.get_register_value(register_y);

                if register_x == register_y {
                    self.program_counter += 2;
                }
            },
            // 6xkk - LD Vx, byte
            // Set Vx to value kk
            0x06 => {
                let register = instruction & 0x0F00 >> 4;
                let value = (instruction & 0x00FF) as u8;
                self.registers[register as usize] = value;
            },
            // 7xkk ADD Vx, byte
            // Set Vx = Vx + kk
            0x07 => {
                let register = instruction & 0x0F00 >> 4;
                let value = (instruction & 0x00FF) as u8;
                self.registers[register as usize] += value;
            },
            0x08 => {
                match instruction & 0x00F {
                    // 8xy0 - LD Vx, Vy
                    // Set Vx to value of Vy
                    0x00 => {
                        let index_x = instruction & 0x0F00 >> 4;
                        let y = instruction & 0x00F0 >> 2;
                        let y = self.get_register_value(y);

                        self.registers[index_x as usize] = y;
                    },
                    // 8xy1 - OR Vx, Vy
                    // Set Vx = Vx OR Vy.
                    0x01 => {
                        let index_x = instruction & 0x0F00 >> 4;
                        let y = instruction & 0x00F0 >> 2;
                        let x = self.get_register_value(index_x);
                        let y = self.get_register_value(y);

                        self.registers[index_x as usize] = x | y;
                    },
                    // 8xy2 - AND Vx, Vy
                    // Set Vx = Vx AND Vy.
                    0x02 => {
                        let index_x = instruction & 0x0F00 >> 4;
                        let y = instruction & 0x00F0 >> 2;
                        let x = self.get_register_value(index_x);
                        let y = self.get_register_value(y);

                        self.registers[index_x as usize] = x & y;
                    },
                    // 8xy3 - XOR Vx, Vy
                    // Set Vx = Vx XOR Vy.
                    0x03 => {
                        let index_x = instruction & 0x0F00 >> 4;
                        let y = instruction & 0x00F0 >> 2;
                        let x = self.get_register_value(index_x);
                        let y = self.get_register_value(y);

                        self.registers[index_x as usize] = x ^ y;
                    },
                    // 8xy4 - ADD Vx, Vy
                    // Set Vx = Vx + Vy, set VF = carry.
                    0x04 => {
                        let index_x = instruction & 0x0F00 >> 4;
                        let y = instruction & 0x00F0 >> 2;
                        let x = self.get_register_value(index_x);
                        let y = self.get_register_value(y);

                        let result = (Wrapping(x as u16) + Wrapping(y as u16)).0;
                        if result > 255 {
                            self.set_carry_flag(1);
                        } else {
                            self.set_carry_flag(0);
                        }

                        self.registers[index_x as usize] = (result & 0xFFFF) as u8;
                    },
                    // 8xy5 - SUB Vx, Vy
                    // Set Vx = Vx - Vy, set VF = NOT borrow.
                    0x05 => {
                        let index_x = instruction & 0x0F00 >> 4;
                        let y = instruction & 0x00F0 >> 2;
                        let x = self.get_register_value(index_x);
                        let y = self.get_register_value(y);

                        if x > y {
                            self.set_carry_flag(1);
                        } else {
                            self.set_carry_flag(0);
                        }

                        self.registers[index_x as usize] = x - y;
                    },
                    // 8xy6 - SHR Vx {, Vy}
                    // Set Vx = Vx SHR 1.
                    0x06 => {
                        let index_x = instruction & 0x0F00 >> 4;
                        let x = self.get_register_value(index_x);

                        if x & 0x0F == 0x01 {
                            self.set_carry_flag(1);
                        } else {
                            self.set_carry_flag(0);
                        }

                        self.registers[index_x as usize] = x / 2;
                    },
                    // 8xy7 - SUBN Vx, Vy
                    // Set Vx = Vy - Vx, set VF = NOT borrow.
                    0x07 => {
                        let index_x = instruction & 0x0F00 >> 4;
                        let y = instruction & 0x00F0 >> 2;
                        let x = self.get_register_value(index_x);
                        let y = self.get_register_value(y);

                        if y > x {
                            self.set_carry_flag(1);
                        } else {
                            self.set_carry_flag(0);
                        }

                        self.registers[index_x as usize] = y- x;
                    },
                    // 8xyE - SHL Vx {, Vy}
                    // Set Vx = Vx SHL 1.
                    0x08 => {
                        let index_x = instruction & 0x0F00 >> 4;
                        let x = self.get_register_value(index_x);

                        if x & 0xF0 == 0x01 {
                            self.set_carry_flag(1);
                        } else {
                            self.set_carry_flag(0);
                        }

                        self.registers[index_x as usize] = x * 2;
                    },
                    _ => panic!("unreachable")
                }


            }

            // 9xy0 - SNE Vx, Vy
            // Skip next instruction if Vx != Vy
            0x09 => {
                let reg_x = instruction & 0x0F00 >> 4;
                let reg_y = instruction & 0x00F0 >> 2;
                let reg_x = self.get_register_value(reg_x);
                let reg_y = self.get_register_value(reg_y);

                if reg_x != reg_y {
                    self.program_counter += 2;
                }
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
                let register = instruction & 0x0F00 >> 4;
                let rng = rand::thread_rng().gen_range(0, 256) as u8;
                let value = (instruction & 0x00FF) as u8;

                self.registers[register as usize] = rng & value;
            },
            // Dxyn - DRW Vx, Vy, nibble
            // Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision
            0x0D => {
                let index_x = instruction & 0x0F00 >> 4;
                let index_y = instruction & 0x00F0 >> 2;
                let bytes_to_read = instruction & 0x000F;

                let mut sprite = Vec::new();
                for x in 0..bytes_to_read {
                    let address = self.i + x;
                    sprite.push(self.memory[address as usize]);
                }

                let x = self.get_register_value(index_x);
                let y = self.get_register_value(index_y);

                self.write_to_screen(x, y, sprite);
            },
            0x0E => {

                match instruction & 0x00FF {
                    // Ex9E - SKP Vx
                    // Skip next instruction if key with the value of Vx is pressed.
                    0x9E => {

                    },
                    // ExA1 - SKNP Vx
                    // Skip next instruction if key with the value of Vx is not pressed.
                    0xA1 => {

                    },
                    _ => panic!("unreachable")
                }
            },
            0x0F => {
                // Fx07 - LD Vx, DT
                // Set Vx = delay timer value.
                match instruction & 0x00FF {
                    // Fx07 - LD Vx, DT
                    // Set Vx = delay timer value.
                    0x07 => {
                        let x = instruction & 0x0F00 >> 4;
                        self.registers[x as usize] = self.delay_timer;
                    },
                    // Fx0A - LD Vx, K
                    // Wait for a key press, store the value of the key in Vx.
                    0x0A => {

                    },
                    // Fx15 - LD DT, Vx
                    // Set delay timer = Vx.
                    0x15 => {
                        let x = instruction & 0x0F00 >> 4;
                        self.delay_timer = self.registers[x as usize];
                    },
                    // Fx18 - LD ST, Vx
                    // Set sound timer = Vx.
                    0x18 => {
                        let x = instruction & 0x0F00 >> 4;
                        self.sound_timer = self.registers[x as usize];
                    },
                    // Fx1E - ADD I, Vx
                    // Set I = I + Vx.
                    0x1E => {
                        let x = instruction & 0x0F00 >> 4;
                        self.i = (Wrapping(self.i) + Wrapping(x)).0;
                    },
                    // Fx29 - LD F, Vx
                    // Set I = location of sprite for digit Vx.
                    0x29 => {

                    },
                    // Fx33 - LD B, Vx
                    // Store BCD representation of Vx in memory locations I, I+1, and I+2.
                    0x33 => {

                    },
                    // Fx55 - LD [I], Vx
                    // Store registers V0 through Vx in memory starting at location I.
                    0x55 => {
                        let range = instruction & 0x0F00 >> 4;
                        for x in 0..range {
                            let val = self.registers[x as usize];
                            self.memory[(self.i + x) as usize] = val;
                        }
                    },
                    // Fx65 - LD Vx, [I]
                    // Read registers V0 through Vx from memory starting at location I.
                    0x65 => {
                        let range = instruction & 0x0F00 >> 4;
                        for x in 0..range {
                            let val = self.memory[(self.i + x) as usize];
                            self.registers[x as usize] = val;
                        }
                    },
                    _ => panic!("Unrecognized subcode in instruction {:x}", instruction)
                }

                let register = instruction & 0x0F00 >> 4;
                self.registers[register as usize] = self.delay_timer;
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