use bus::Bus;
use cart::Rom;
use cpu::CPU;

use crate::trace::trace;

mod bus;
mod cart;
mod cpu;
mod opcodes;
mod trace;

fn main() {
    let bytes: Vec<u8> = std::fs::read("nestest.nes").unwrap();
    let rom = Rom::new(&bytes).unwrap();

    let bus = Bus::new(rom);
    let mut cpu = CPU::new(bus);
    cpu.reset();
    cpu.program_counter = 0xC000;
    cpu.run(move |cpu| {
        println!("{}", trace(cpu));
    });
}
