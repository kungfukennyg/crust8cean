use rand::Rng;
use minifb::{WindowOptions, Window, Scale, Key};
use std::num::Wrapping;
use std::time::{Instant, Duration};
use std::thread;
use std::ops::Sub;
use crate::modules::display::{MiniFbDisplay, FONT_ARRAY_SIZE, FONT_SPRITES, BYTES_PER_CHARACTER};
use crate::modules::input::Keymap;
use crate::modules::audio;
use crate::modules::config::Config;

const MEMORY_SIZE: u16 = 4096;
const SCREEN_WIDTH: u16 = 64;
const SCREEN_HEIGHT: u16 = 32;

pub const PROGRAM_COUNTER_START_ADDR: u16 = 0x200;

// 60hz, needed for sound/delay timer
const SOUND_DELAY_TICK_RATE: Duration = Duration::from_millis(
    ((1 as f64 / 120 as f64) * 1000 as f64) as u64);
// this length seems to work for other emulators :shrug:
const MAIN_TICK_RATE: Duration = Duration::from_millis(2);

pub struct Cpu {
    // memory
    memory: [u8; MEMORY_SIZE as usize],
    // registers (V0-VF)
    pub registers: [u8; 16],
    pub i: usize, // address register

    pub program_counter: u16,

    // stack
    pub stack: [u16; 16],
    pub stack_pointer: u8,

    // timers
    pub delay_timer: u8,
    pub sound_timer: u8,

    // display
    display: MiniFbDisplay,

    // input
    keypad: Keymap,

    // window
    window: Window,

    // interpreter specific
    dead: bool,
    last_cycle: Instant,
    total_cycles: u64,
    config: Config,
}

impl Cpu {
    pub fn new(program: &Vec<u8>, config: Config) -> Self {
        let mut cpu = Cpu {
            memory: [0; MEMORY_SIZE as usize],
            registers: [0; 16],
            i: 0,
            program_counter: PROGRAM_COUNTER_START_ADDR,
            stack: [0; 16],
            stack_pointer: 0,
            delay_timer: 0,
            sound_timer: 0,
            display: MiniFbDisplay::new(config.initial_color),
            keypad: Keymap::new(),

            window: Window::new("crust8cean - ESC to exit",
                                SCREEN_WIDTH as usize, SCREEN_HEIGHT as usize,
                                WindowOptions {
                                    borderless: false,
                                    title: true,
                                    resize: true,
                                    scale: Scale::FitScreen,
                                }).unwrap_or_else(|e| {
                                        println!("{}", e);
                                        panic!("{}", e);
            }),
            last_cycle: Instant::now().sub(SOUND_DELAY_TICK_RATE),
            total_cycles: 0,
            dead: false,
            config,
        };

        // init fonts
        for (x, line) in FONT_SPRITES.iter().enumerate() {
            cpu.memory[x] = *line;
        }
        println!("Initialized {} font sprites", FONT_ARRAY_SIZE);

        // init program
        let start_addr = cpu.program_counter as usize;
        for (i, val) in program.iter().enumerate() {
            cpu.write(start_addr + i, *val);
        }
        println!("Read program of {} bytes into memory", program.len());

        // init window, so we don't have to wait until a redraw is triggered
        cpu.display.set_redraw(true);
        cpu
    }

    pub fn run(&mut self) {
        if self.dead {
            return;
        }

        if !self.window.is_open() {
            self.die();
        }

        self.keypad.update(&self.window);

        // handle interpreter specific keys
        let interpreter_specific_keys = self.keypad.get_interpreter_keys_pressed().clone();
        for key in interpreter_specific_keys.iter() {
            match key {
                // palette swap
                Key::P => {
                    self.display.change_color();
                }
                // exit
                Key::Escape => {
                    self.die();
                },
                _ => ()
            }
        }
        self.keypad.clear_interpreter_keys_pressed();

        // wait for key press
        if self.keypad.is_awaiting_keypress() {
            let key_pressed = self.keypad.await_keypress();
            match key_pressed {
                Some(key) => self.registers[self.keypad.get_awaiting_keypress_register()] = key,
                None => ()
            }
        } else {
            // run instruction
            self.emulate_cycle();

            // graphics
            self.display.render(&mut self.window);
        }

        let now = Instant::now();
        if now.duration_since(self.last_cycle).as_millis() >= SOUND_DELAY_TICK_RATE.as_millis() {
        // decrement on 60hz timer
            if self.delay_timer > 0 {
                self.delay_timer -= 1;
            }
            if self.sound_timer > 0 {
                self.sound_timer -= 1;
                if self.config.play_sound {
                    audio::play_tone();
                }
            }

            self.last_cycle = Instant::now();
        }

        self.total_cycles += 1;
        thread::sleep(MAIN_TICK_RATE);
    }

    pub fn pop(&mut self) -> u16 {
        self.stack_pointer -= 1;
        let val = self.stack[self.stack_pointer as usize];
        val
    }

    pub fn push(&mut self, value: u16) {
        self.stack[self.stack_pointer as usize] = value;
        self.stack_pointer += 1;
    }

    pub fn is_running(&self) -> bool {
        !self.dead
    }

    pub fn get_total_cycles(&self) -> u64 {
        self.total_cycles
    }

    pub fn get_times_screen_rendered(&self) -> u64 {
        self.display.get_times_rendered()
    }

    fn read(&self, address: u16) -> u8 {
        if address > self.memory.len() as u16 {
            panic!("Read at 0x{:x} out of bounds", address);
        }

        self.memory[address as usize]
    }

    fn read_word(&self, address: u16) -> u16 {
        let low = self.read(address);
        let high = self.read(address + 1);

        ((low as u16) << 8) | (high as u16)
    }

    fn write(&mut self, address: usize, value: u8) {
        if address > self.memory.len() as usize {
            panic!("write at {:x} out of bounds", address);
        }
        self.memory[address] = value;
    }

    /// shuts down the emulator
    fn die(&mut self) {
        self.dead = true;
    }

    fn set_carry_flag(&mut self, value: u8) {
        self.registers[0x0F] = value;
    }

    // ingenious nibble matching borrowed from https://github.com/starrhorne/chip8-rust/blob/master/src/processor.rs#L120
    fn emulate_cycle(&mut self) {
        let pc = self.program_counter;

        // the full 16 bits of an instruction, including operands
        let opcode = self.read_word(pc);
        self.program_counter += 2;

        // opcodes are stored in the first 2 bits of an instruction, big endian
        let nibbles = (
            ((opcode & 0xF000) >> 12) as u8,
            ((opcode & 0x0F00) >> 8) as u8,
            ((opcode & 0x00F0) >> 4) as u8,
            (opcode & 0x000F) as u8
        );

        let nnn = opcode & 0x0FFF;
        let kk = (opcode & 0x00FF) as u8;
        let x = nibbles.1 as usize;
        let y = nibbles.2 as usize;
        let z = nibbles.3 as usize;
        match nibbles {
            // 0nnn - SYS addr
            // Jump to routine at nnn
            (0x00, 0x00, 0x00, 0x00) => {
                println!("0nnn");
                // not used in interpreters
            },
            // 00E0 - CLS
            // Clear display
            (0x00, 0x00, 0x0E, 0x00) => {
                println!("CLS");
                self.display.clear();
            },
            // 00EE - RET
            // Return from a subroutine i.e. set pc to top of stack
            (0x00, 0x00, 0x0E, 0x0E) => {
                println!("RET");
                self.program_counter = self.pop();
            },
            // 1nnn - JMP addr
            // Jump to address nnn
            (0x01, _, _, _) => {
                println!("JMP {:x}", nnn);
                // check for infinite jump loop
                if self.read_word(nnn) == opcode {
                    self.die();
                }
                self.program_counter = nnn;

            },
            // 2nnn - CALL addr
            // Call subroutine at nnn
            (0x02, _, _, _) => {
                println!("CALL {:x}", nnn);
                let pc = self.program_counter;
                self.push(pc);
                self.program_counter = nnn;
            },
            // 3xkk - SE Vx, byte
            // Skip next instruction if Vx == kk
            (0x03, _, _, _) => {
                println!("SE V{:x}, {:x}", x, kk);
                let x = self.registers[x];
                if x == kk {
                    self.program_counter += 2;
                }
            },
            // 4xkk - SNE Vx, byte
            // Skip next instruction if Vx != kk
            (0x04, _, _, _) => {
                println!("SNE V{:x}, {:x}", x, kk);
                let x = self.registers[x];
                if x != kk {
                    self.program_counter += 2;
                }
            },
            // 5xy0 - SE Vx, Vy
            // Skip next instruction if Vx == Vy
            (0x05, _, _, 0x00) => {
                println!("SE V{:x}, V{:x}", x, y);
                let x = self.registers[x];
                let y = self.registers[y];

                if x == y {
                    self.program_counter += 2;
                }
            },
            // 6xkk - LD Vx, byte
            // Set Vx to value kk
            // Set Vx to value kk
            (0x06, _, _, _) => {
                println!("LD V{:x}, {:x}", x, kk);
                self.registers[x] = kk;
            },
            // 7xkk ADD Vx, byte
            // Set Vx = Vx + kk
            (0x07, _, _, _) => {
                println!("ADD V{:X}, {:x}", x, kk);
                self.registers[x] = (Wrapping(self.registers[x]) + Wrapping(kk)).0;
            },
            // 8xy0 - LD Vx, Vy
            // Set Vx to value of Vy
            (0x08, _, _, 0x00) => {
                println!("LD V{:x}, V{:x}", x, y);
                let y = self.registers[y];
                self.registers[x] = y;
            },
            // 8xy1 - OR Vx, Vy
            // Set Vx = Vx OR Vy.
            (0x08, _, _, 0x01) => {
                println!("OR V{:x}, V{:x}", x, y);
                self.registers[x] = self.registers[x] | self.registers[y];
            },
            // 8xy2 - AND Vx, Vy
            // Set Vx = Vx AND Vy.
            (0x08, _, _, 0x02) => {
                println!("AND V{:x}, V{:x}", x, y);
                self.registers[x] = self.registers[x] & self.registers[y];
            },
            // 8xy3 - XOR Vx, Vy
            // Set Vx = Vx XOR Vy.
            (0x08, _, _, 0x03) => {
                println!("XOR V{:x}, V{:x}", x, y);
                self.registers[x] = self.registers[x] ^ self.registers[y];
            },
            // 8xy4 - ADD Vx, Vy
            // Set Vx = Vx + Vy, set VF = carry.
            (0x08, _, _, 0x04) => {
                println!("ADD V{:x}, V{:x}", x, y);
                let ret = (Wrapping(self.registers[x] as u16)
                    + Wrapping(self.registers[y] as u16)).0;
                self.set_carry_flag(if ret > 0xFF { 1 } else { 0 });
                // lower 8 bits
                self.registers[x] = (ret & 0x00FF) as u8;
            },
            // 8xy5 - SUB Vx, Vy
            // Set Vx = Vx - Vy, set VF = NOT borrow.
            (0x08, _, _, 0x05) => {
                println!("SUB V{:x}, V{:X}", x, y);
                let x_val = self.registers[x];
                let y_val = self.registers[y];
                self.set_carry_flag(if x_val > y_val { 1 } else { 0 });
                self.registers[x] = x_val.wrapping_sub(y_val);
            },
            // 8xy6 - SHR Vx, Vy
            // Set Vx = Vx SHR 1.
            (0x08, _, _, 0x06) => {
                println!("SHR V{:x}, V{:x}", x, y);
                self.set_carry_flag(self.registers[x] & 0x1);
                self.registers[x] >>= 1;
            },
            // 8xy7 - SUBN Vx, Vy
            // Set Vx = Vy - Vx, set VF = NOT borrow.
            (0x08, _, _, 0x07) => {
                println!("SUBN V{:x}, V{:x}", x, y);
                let x_val = self.registers[x];
                let y_val = self.registers[y];
                self.set_carry_flag(if y_val > x_val { 1 } else { 0 });
                self.registers[x] = y_val.wrapping_sub(x_val);
            },
            // 8xyE - SHL Vx, Vy
            // Set Vx = Vx SHL 1.
            (0x08, _, _, 0x0E) => {
                println!("SHL V{:x}, V{:x}", x, y);
                self.set_carry_flag((self.registers[x] & 0b10000000) >> 7);
                self.registers[x] <<= 1;
            },
            // 9xy0 - SNE Vx, Vy
            // Skip next instruction if Vx != Vy
            (0x09, _, _, 0x00) => {
                println!("SNE V{:x}, V{:x}", x, y);
                if self.registers[x] != self.registers[y] {
                    self.program_counter += 2;
                }
            },
            // Annn - LD I, addr
            // Set register I to nnn
            (0x0A, _, _, _) => {
                println!("LD I, {:X}", nnn);
                self.i = nnn as usize;
            },
            // Bnnn - JP V0, addr
            // Jump to location nnn + V0
            (0x0B, _, _, _) => {
                println!("JP V0, {:x}", nnn);
                self.program_counter = self.registers[0] as u16 + nnn
            },
            // Cxkk - RND Vx, byte
            // Set Vx = random byte (0-255) AND kk
            (0x0C, _, _, _) => {
                println!("RND V{:x}, rng", x);
                let rng = rand::thread_rng().gen_range(0, 256) as u8;
                self.registers[x] = rng & kk;
            },
            // Dxyn - DRW Vx, Vy, nibble
            // Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision
            (0x0D, _, _, _) => {
                println!("DXYN V{:x}, V{:x} {}", x, y, z);
                let sprite_height = z;
                let x = self.registers[x] as usize;
                let y = self.registers[y] as usize;

                self.set_carry_flag(0);
                let mut collision = false;
                {
                    for height in 0..sprite_height {
                        // get current sprite stored in i + offset (by instr fx29)
                        let cur_line = self.memory[self.i + height];
                        // iterate each bit in byte
                        for bit in 0..8 {
                            // check if current bit (pixel) is set, if it is we xor it with existing
                            if cur_line & (0x80 >> bit) != 0 {
                                let pos_x = (x + bit) as u16 % SCREEN_WIDTH;
                                let pos_y = (y + height) as u16 % SCREEN_HEIGHT;
                                let pos = (pos_x + (pos_y * SCREEN_WIDTH)) as usize;
                                if self.display.read(pos) == 1 {
                                    collision = true;
                                }
                                self.display.write(pos, self.display.read(pos) ^ 1);
                            }
                        }
                    }
                }

                if collision {
                    self.set_carry_flag(1);
                    self.display.set_redraw(true);
                }
            },
            // Ex9E - SKP Vx
            // Skip next instruction if key with the value of Vx is pressed.
            (0x0E, _, 0x09, 0x0E) => {
                println!("SKP V{:x}", x);
                let x = self.registers[x] as usize;
                if self.keypad.is_key_pressed(x) {
                    self.program_counter += 2;
                }
            },
            // ExA1 - SKNP Vx
            // Skip next instruction if key with the value of Vx is not pressed.
            (0x0E, _, 0x0A, 0x01) => {
                println!("SKNP V{:x}", x);
                let x = self.registers[x] as usize;
                if !self.keypad.is_key_pressed(x) {
                    self.program_counter += 2;
                }
            }
            // Fx07 - LD Vx, DT
            // Set Vx = delay timer value.
            (0x0F, _, _, 0x07) => {
                println!("LD V{:x} DT", x);
                self.registers[x] = self.delay_timer;
            },
            // Fx0A - LD Vx, K
            // Wait for a key press, store the value of the key in Vx.
            (0x0F, _, _, 0x0A) => {
                println!("LD V{} K", x);
                self.keypad.set_awaiting_keypress(true);
                self.keypad.set_awaiting_keypress_register(x);
            },
            // Fx15 - LD DT, Vx
            // Set delay timer = Vx.
            (0x0F, _, 0x01, 0x05) => {
                println!("LD DT V{:x}", x);
                self.delay_timer = self.registers[x]
            },
            // Fx18 - LD ST, Vx
            // Set sound timer = Vx.
            (0x0F, _, 0x01, 0x08) => {
                println!("LD ST V{:x}", x);
                self.sound_timer = self.registers[x];
            },
            // Fx1E - ADD I, Vx
            // Set I = I + Vx.
            (0x0F, _, 0x01, 0x0E) => {
                println!("ADD I V{:x}", x);
                self.i += self.registers[x] as usize;
                self.set_carry_flag(if self.i > 0x0F00 { 1 } else { 0 });
            },
            // Fx29 - LD F, Vx
            // Set I = location of sprite for digit Vx.
            (0x0F, _, 0x02, 0x09) => {
                println!("LD F V{:x}", x);
                self.i = (self.registers[x] as usize) * BYTES_PER_CHARACTER as usize;
            },
            // Fx33 - LD B, Vx
            // Store BCD representation of Vx in memory locations I, I+1, and I+2.
            (0x0F, _, 0x03, 0x03) => {
                println!("LD B V{:x}", x);
                let x = self.registers[x];

                self.memory[self.i] = x / 100;
                self.memory[(self.i + 1)] = (x % 100) / 10;
                self.memory[(self.i + 2)] = x % 10;
            },
            // Fx55 - LD [I], Vx
            // Store registers V0 through Vx in memory starting at location I.
            (0x0F, _, 0x05, 0x05) => {
                println!("LD [I] V{:x}", x);
                for i in 0..x + 1 {
                    let val = self.registers[i];
                    self.memory[self.i + i] = val;
                }
            },
            // Fx65 - LD Vx, [I]
            // Read registers V0 through Vx from memory starting at location I.
            (0x0F, _, 0x06, 0x05) => {
                println!("LD V{:x} [I]", x);
                for i in 0..x + 1 {
                    self.registers[i] = self.memory[self.i + i];
                }
            },
            _ => panic!("Unrecognized nibbles ({:x}, {:x}, {:x}, {:x})", nibbles.0, nibbles.1, nibbles.2, nibbles.3)
        }
        if self.config.debug {
            println!("---Registers---");
            println!("V0: {:x}, V1: {:x}, V2: {:x}, V3: {:x}, V4: {:x}, V5: {:x}, V6: {:x}, V7: {:x}",
                     self.registers[0], self.registers[1], self.registers[2], self.registers[3],
                     self.registers[4], self.registers[5], self.registers[6], self.registers[7]);
            println!("V8: {:x}, V9: {:x}, VA: {:x}, VB: {:x}, VC: {:x}, VD: {:x}, VE: {:x}, VF: {:x}",
                     self.registers[8], self.registers[9], self.registers[10], self.registers[11],
                     self.registers[12], self.registers[13], self.registers[14], self.registers[15]);
            println!("I: {:x}", self.i);
            println!("SP: {:x}", self.stack_pointer);
            println!("DT: {}", self.delay_timer);
            println!("ST: {}", self.sound_timer);
            println!("---Keys Pressed---");
            println!("{:?}", self.keypad.map_keys_pressed_to_real_values());
            println!();
        }
    }
}