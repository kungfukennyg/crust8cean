extern crate rand;

mod chip8;

use chip8::cpu::Cpu;

fn main() {
    let cpu = Cpu::new();
    println!("Hello, world! {:?}", cpu);
}
