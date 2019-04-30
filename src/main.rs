extern crate rand;
extern crate minifb;

mod chip8;
mod tests;
mod modules;

use chip8::cpu::Cpu;
use std::{env, thread};
use std::io::Read;
use std::io::Result;
use std::fs::File;
use std::path::Path;
use std::time::Duration;

fn main() {
    let rom_path = env::args().skip(1).next().expect("Failed to find rom file");
    let rom = read_rom(&mut File::open(&Path::new(&rom_path)).unwrap())
        .expect("rom not found");

    let mut cpu = Cpu::new(&rom);
    println!("crust8cean starting...");
    thread::sleep(Duration::from_millis(3000));

    loop {
        if !cpu.is_running() {
            break;
        }
        cpu.run();
    }

    println!();
    println!("Program finished!");
    // TODO print some stats?

    println!();
    println!("Total cycles emulated: {}", cpu.get_total_cycles());
    println!("Times screen drawn: {}", cpu.get_times_screen_rendered());
}

fn read_rom(r: &mut Read) -> Result<Vec<u8>> {
    let mut data = Vec::new();
    r.read_to_end(&mut data)?;

    return Ok(data);
}
