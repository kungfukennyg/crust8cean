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

    }
}

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TODO cpu fmt")
    }
}