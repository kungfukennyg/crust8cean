extern crate rand;
extern crate minifb;
extern crate ears;
extern crate config;

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
use crate::modules::config::Config;

fn main() {
    let rom_path = env::args().skip(1).next().expect("Usage: ./crust8cean <path-to-rom>");
    let rom = read_rom(&mut File::open(&Path::new(&rom_path)).unwrap())
        .expect("rom not found");

    let config = Config::new("config");
    println!("read config: {:?}", config);
    let mut cpu = Cpu::new(&rom, config);
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
