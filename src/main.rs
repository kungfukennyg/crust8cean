extern crate rand;
extern crate minifb;

mod chip8;
mod tests;

use chip8::cpu::Cpu;
use std::env;
use std::io::Read;
use std::io::Result;
use std::fs::File;
use std::path::Path;

fn main() {
    let rom_path = env::args().skip(1).next().expect("Failed to find rom file");
    let rom = read_rom(&mut File::open(&Path::new(&rom_path)).unwrap())
        .expect("rom not found");

    let mut cpu = Cpu::new(&rom);

    loop {
        if !cpu.is_window_open() {
            break;
        }
        cpu.run();
    }
}

fn read_rom(r: &mut Read) -> Result<Vec<u8>> {
    let mut data = Vec::new();
    r.read_to_end(&mut data)?;

    return Ok(data);
}
