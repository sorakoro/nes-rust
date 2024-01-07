use cpu::CPU;

mod bus;
mod cpu;
mod opcodes;

fn main() {
    let mut cpu = CPU::new();
    cpu.reset();
    cpu.run();
}
