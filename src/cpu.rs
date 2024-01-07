use crate::opcodes;
use std::collections::HashMap;

const NEGATIVE_FLAG: u8 = 0b1000_0000;
const OVERFLOW_FLAG: u8 = 0b0100_0000;
const BREAK2_FLAG: u8 = 0b0010_0000;
const BREAK_FLAG: u8 = 0b0001_0000;
const DECIMAL_FLAG: u8 = 0b0000_1000;
const INTERRUPT_DISABLE_FLAG: u8 = 0b0000_0100;
const ZERO_FLAG: u8 = 0b0000_0010;
const CARRY_FLAG: u8 = 0b0000_0001;

#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum AddressingMode {
    Implied,
    Accumulator,
    Immediate,
    ZeroPage,
    ZeroPage_X,
    ZeroPage_Y,
    Absolute,
    Absolute_X,
    Absolute_Y,
    Indirect,
    Indirect_X,
    Indirect_Y,
    RELATIVE,
}

pub trait Mem {
    fn mem_read(&mut self, addr: u16) -> u8;

    fn mem_write(&mut self, addr: u16, value: u8);

    fn mem_read_u16(&mut self, pos: u16) -> u16 {
        let lo = self.mem_read(pos) as u16;
        let hi = self.mem_read(pos + 1) as u16;
        (hi << 8) | (lo as u16)
    }

    fn mem_write_u16(&mut self, pos: u16, value: u16) {
        let hi = (value >> 8) as u8;
        let lo = (value & 0x00FF) as u8;
        self.mem_write(pos, lo);
        self.mem_write(pos + 1, hi);
    }
}

pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: u8,
    pub stack_pointer: u8,
    pub program_counter: u16,
    memory: [u8; 0xFFFF],
}

impl Mem for CPU {
    fn mem_read(&mut self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn mem_write(&mut self, addr: u16, value: u8) {
        self.memory[addr as usize] = value;
    }

    fn mem_read_u16(&mut self, pos: u16) -> u16 {
        if pos == 0xFF || pos == 0x02FF {
            let lo = self.mem_read(pos);
            let hi = self.mem_read(pos & 0xFF00);
            return (hi as u16) << 8 | (lo as u16);
        }
        self.mem_read_u16(pos)
    }

    fn mem_write_u16(&mut self, pos: u16, value: u16) {
        self.mem_write_u16(pos, value);
    }
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: 0x24,
            stack_pointer: 0xFD,
            program_counter: 0,
            memory: [0; 0xFFFF],
        }
    }

    pub fn get_absolute_address(&mut self, mode: &AddressingMode, addr: u16) -> u16 {
        match mode {
            AddressingMode::Implied => {
                panic!("AddressingMode::Implied");
            }
            AddressingMode::Accumulator => {
                panic!("AddressingMode::Accumulator");
            }
            AddressingMode::Immediate => addr,

            AddressingMode::ZeroPage => self.mem_read(addr) as u16,

            AddressingMode::Absolute => self.mem_read_u16(addr),

            AddressingMode::ZeroPage_X => {
                let pos = self.mem_read(addr);
                pos.wrapping_add(self.register_x) as u16
            }
            AddressingMode::ZeroPage_Y => {
                let pos = self.mem_read(addr);
                pos.wrapping_add(self.register_y) as u16
            }
            AddressingMode::Absolute_X => {
                let base = self.mem_read_u16(addr);
                base.wrapping_add(self.register_x as u16)
            }
            AddressingMode::Absolute_Y => {
                let base = self.mem_read_u16(addr);
                base.wrapping_add(self.register_y as u16)
            }
            AddressingMode::Indirect => {
                let base = self.mem_read_u16(addr);
                self.mem_read_u16(base)
            }
            AddressingMode::Indirect_X => {
                let base = self.mem_read(addr);
                let ptr = (base as u8).wrapping_add(self.register_x);
                self.mem_read_u16(ptr as u16)
            }
            AddressingMode::Indirect_Y => {
                let base = self.mem_read(addr);
                let deref_base = self.mem_read_u16(base as u16);
                deref_base.wrapping_add(self.register_y as u16)
            }
            AddressingMode::RELATIVE => {
                let jump = self.mem_read(addr) as i8;
                addr.wrapping_add(1).wrapping_add(jump as u16)
            }
        }
    }

    fn get_operand_address(&mut self, mode: &AddressingMode) -> u16 {
        self.get_absolute_address(mode, self.program_counter)
    }

    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.status = 0x24;
        self.stack_pointer = 0xFD;
        self.program_counter = self.mem_read_u16(0xFFFC);
    }

    pub fn run(&mut self) {
        let ref opcodes: HashMap<u8, &'static opcodes::OpCode> = *opcodes::OPCODES_MAP;

        loop {
            let opscode = self.mem_read(self.program_counter);
            self.program_counter += 1;
            let program_counter_state = self.program_counter;

            let opcode = opcodes
                .get(&opscode)
                .expect(&format!("OpCode {:x} is not recognized", opscode));

            match opscode {
                0x69 | 0x65 | 0x75 | 0x6D | 0x7D | 0x79 | 0x61 | 0x71 => {
                    self.adc(&opcode.mode);
                }
                0xE9 | 0xE5 | 0xF5 | 0xED | 0xFD | 0xF9 | 0xE1 | 0xF1 => {
                    self.sbc(&opcode.mode);
                }
                0x29 | 0x25 | 0x35 | 0x2D | 0x3D | 0x39 | 0x21 | 0x31 => {
                    self.and(&opcode.mode);
                }
                0x09 | 0x05 | 0x15 | 0x0D | 0x1D | 0x19 | 0x01 | 0x11 => {
                    self.ora(&opcode.mode);
                }
                0x49 | 0x45 | 0x55 | 0x4D | 0x5D | 0x59 | 0x41 | 0x51 => {
                    self.eor(&opcode.mode);
                }
                0x0A | 0x06 | 0x16 | 0x0E | 0x1E => {
                    self.asl(&opcode.mode);
                }
                0x4A | 0x46 | 0x56 | 0x4E | 0x5E => {
                    self.lsr(&opcode.mode);
                }
                0x2A | 0x26 | 0x36 | 0x2E | 0x3E => {
                    self.rol(&opcode.mode);
                }
                0x6A | 0x66 | 0x76 | 0x6E | 0x7E => {
                    self.ror(&opcode.mode);
                }
                0x90 => {
                    self.bcc(&opcode.mode);
                }
                0xB0 => {
                    self.bcs(&opcode.mode);
                }
                0xF0 => {
                    self.beq(&opcode.mode);
                }
                0xD0 => {
                    self.bne(&opcode.mode);
                }
                0x30 => {
                    self.bmi(&opcode.mode);
                }
                0x10 => {
                    self.bpl(&opcode.mode);
                }
                0x50 => {
                    self.bvc(&opcode.mode);
                }
                0x70 => {
                    self.bvs(&opcode.mode);
                }
                0x24 | 0x2C => {
                    self.bit(&opcode.mode);
                }
                0x4C | 0x6C => {
                    self.jmp(&opcode.mode);
                }
                0x20 => {
                    self.jsr(&opcode.mode);
                }
                0x60 => {
                    self.rts();
                }
                0x00 => {
                    return;
                }
                0x40 => {
                    self.rti();
                }
                0xC9 | 0xC5 | 0xD5 | 0xCD | 0xDD | 0xD9 | 0xC1 | 0xD1 => {
                    self.cmp(&opcode.mode);
                }
                0xE0 | 0xE4 | 0xEC => {
                    self.cpx(&opcode.mode);
                }
                0xC0 | 0xC4 | 0xCC => {
                    self.cpy(&opcode.mode);
                }
                0xC6 | 0xD6 | 0xCE | 0xDE => {
                    self.dec(&opcode.mode);
                }
                0xCA => {
                    self.dex();
                }
                0x88 => {
                    self.dey();
                }
                0xE6 | 0xF6 | 0xEE | 0xFE => {
                    self.inc(&opcode.mode);
                }
                0xE8 => {
                    self.inx();
                }
                0xC8 => {
                    self.iny();
                }
                0x18 => {
                    self.clc();
                }
                0x38 => {
                    self.sec();
                }
                0xD8 => {
                    self.cld();
                }
                0xF8 => {
                    self.sed();
                }
                0x58 => {
                    self.cli();
                }
                0x78 => {
                    self.sei();
                }
                0xB8 => {
                    self.clv();
                }
                0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => {
                    self.lda(&opcode.mode);
                }
                0xA2 | 0xA6 | 0xB6 | 0xAE | 0xBE => {
                    self.ldx(&opcode.mode);
                }
                0xA0 | 0xA4 | 0xB4 | 0xAC | 0xBC => {
                    self.ldy(&opcode.mode);
                }
                0x85 | 0x95 | 0x8D | 0x9D | 0x99 | 0x81 | 0x91 => {
                    self.sta(&opcode.mode);
                }
                0x86 | 0x96 | 0x8E => {
                    self.stx(&opcode.mode);
                }
                0x84 | 0x94 | 0x8C => {
                    self.sty(&opcode.mode);
                }
                0xAA => {
                    self.tax();
                }
                0xA8 => {
                    self.tay();
                }
                0xBA => {
                    self.tsx();
                }
                0x8A => {
                    self.txa();
                }
                0x9A => {
                    self.txs();
                }
                0x98 => {
                    self.tya();
                }
                0x48 => {
                    self.pha();
                }
                0x68 => {
                    self.pla();
                }
                0x08 => {
                    self.php();
                }
                0x28 => {
                    self.plp();
                }
                0xEA | 0x04 | 0x44 | 0x64 | 0x14 | 0x34 | 0x54 | 0x74 | 0xD4 | 0xF4 | 0x0C
                | 0x1C | 0x3C | 0x5C | 0x7C | 0xDC | 0xFC | 0x1A | 0x3A | 0x5A | 0x7A | 0xDA
                | 0xFA => {
                    self.nop();
                }
                0x80 => {
                    self.nop();
                }
                0xA7 | 0xB7 | 0xAF | 0xBF | 0xA3 | 0xB3 => {
                    self.lax(&opcode.mode);
                }
                _ => todo!(""),
            }

            if program_counter_state == self.program_counter {
                self.program_counter += (opcode.length - 1) as u16;
            }
        }
    }

    fn adc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        let carry = self.status & 0x01;
        let sum = (self.register_a as u16)
            .wrapping_add(value as u16)
            .wrapping_add(carry as u16);

        let carry = sum > 0xFF;
        if carry {
            self.status |= CARRY_FLAG;
        } else {
            self.status &= !CARRY_FLAG;
        }

        let result = sum as u8;
        if (self.register_a ^ result) & (value ^ result) & 0x80 != 0 {
            self.status |= OVERFLOW_FLAG;
        } else {
            self.status &= !OVERFLOW_FLAG;
        }

        self.register_a = result;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn sbc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        let carry = self.status & 0x01;
        let diff: u16 = (self.register_a as u16)
            .wrapping_sub(value as u16)
            .wrapping_sub((1 - carry) as u16);

        let carry: bool = diff > 0xFF;
        if carry {
            self.status &= !CARRY_FLAG;
        } else {
            self.status |= CARRY_FLAG;
        }

        let result: u8 = diff as u8;
        if (self.register_a & 0x80) != (value & 0x80) && (self.register_a & 0x80) != (result & 0x80)
        {
            self.status |= OVERFLOW_FLAG;
        } else {
            self.status &= !OVERFLOW_FLAG;
        }

        self.register_a = result;
        self.update_zero_and_negative_flags(result);
    }

    fn and(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        self.register_a = self.register_a & value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn ora(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        self.register_a = self.register_a | value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn eor(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        self.register_a = self.register_a ^ value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn asl(&mut self, mode: &AddressingMode) {
        let (value, carry) = if mode == &AddressingMode::Accumulator {
            let (value, carry) = self.register_a.overflowing_mul(2);
            self.register_a = value;
            (value, carry)
        } else {
            let addr = self.get_operand_address(mode);
            let value = self.mem_read(addr);
            let (value, carry) = value.overflowing_mul(2);
            self.mem_write(addr, value);
            (value, carry)
        };

        if carry {
            self.status |= CARRY_FLAG;
        } else {
            self.status &= !CARRY_FLAG;
        }
        self.update_zero_and_negative_flags(value);
    }

    fn lsr(&mut self, mode: &AddressingMode) {
        let (value, carry) = if mode == &AddressingMode::Accumulator {
            let carry = self.register_a & 0x01;
            self.register_a = self.register_a / 2;
            (self.register_a, carry)
        } else {
            let addr = self.get_operand_address(mode);
            let value = self.mem_read(addr);
            let carry = value & 0x01;
            let value = value / 2;
            self.mem_write(addr, value);
            (value, carry)
        };

        if carry != 0 {
            self.status |= CARRY_FLAG;
        } else {
            self.status &= !CARRY_FLAG;
        }
        self.update_zero_and_negative_flags(value);
    }

    fn rol(&mut self, mode: &AddressingMode) {
        let (value, carry) = if mode == &AddressingMode::Accumulator {
            let (value, carry) = self.register_a.overflowing_mul(2);
            self.register_a = value | (self.status & 0x01);
            (self.register_a, carry)
        } else {
            let addr = self.get_operand_address(mode);
            let value = self.mem_read(addr);
            let (value, carry) = value.overflowing_mul(2);
            let value = value | (self.status & 0x01);
            self.mem_write(addr, value);
            (value, carry)
        };

        if carry {
            self.status |= CARRY_FLAG;
        } else {
            self.status &= !CARRY_FLAG;
        }
        self.update_zero_and_negative_flags(value);
    }

    fn ror(&mut self, mode: &AddressingMode) {
        let (value, carry) = if mode == &AddressingMode::Accumulator {
            let carry = self.register_a & 0x01;
            self.register_a = self.register_a / 2;
            self.register_a = self.register_a | ((self.status & 0x01) << 7);
            (self.register_a, carry)
        } else {
            let addr = self.get_operand_address(mode);
            let value = self.mem_read(addr);
            let carry = value & 0x01;
            let value = value / 2;
            let value = value | ((self.status & 0x01) << 7);
            self.mem_write(addr, value);
            (value, carry)
        };

        if carry != 0 {
            self.status |= CARRY_FLAG;
        } else {
            self.status &= !CARRY_FLAG;
        }
        self.update_zero_and_negative_flags(value);
    }

    fn bcc(&mut self, mode: &AddressingMode) {
        if self.status & 0x01 == 0 {
            let addr = self.get_operand_address(mode);
            self.program_counter = addr;
        }
    }

    fn bcs(&mut self, mode: &AddressingMode) {
        if self.status & 0x01 != 0 {
            let addr = self.get_operand_address(mode);
            self.program_counter = addr;
        }
    }

    fn beq(&mut self, mode: &AddressingMode) {
        if self.status & 0x02 != 0 {
            let addr = self.get_operand_address(mode);
            self.program_counter = addr;
        }
    }

    fn bne(&mut self, mode: &AddressingMode) {
        if self.status & 0x02 == 0 {
            let addr = self.get_operand_address(mode);
            self.program_counter = addr;
        }
    }

    fn bmi(&mut self, mode: &AddressingMode) {
        if self.status & 0x80 != 0 {
            let addr = self.get_operand_address(mode);
            self.program_counter = addr;
        }
    }

    fn bpl(&mut self, mode: &AddressingMode) {
        if self.status & 0x80 == 0 {
            let addr = self.get_operand_address(mode);
            self.program_counter = addr;
        }
    }

    fn bvc(&mut self, mode: &AddressingMode) {
        if self.status & 0x40 == 0 {
            let addr = self.get_operand_address(mode);
            self.program_counter = addr;
        }
    }

    fn bvs(&mut self, mode: &AddressingMode) {
        if self.status & 0x40 != 0 {
            let addr = self.get_operand_address(mode);
            self.program_counter = addr;
        }
    }

    fn bit(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        let result = self.register_a & value;

        if result == 0 {
            self.status = self.status | ZERO_FLAG;
        } else {
            self.status = self.status & !ZERO_FLAG
        }

        self.status = (self.status & !(NEGATIVE_FLAG | OVERFLOW_FLAG))
            | (value & (NEGATIVE_FLAG | OVERFLOW_FLAG))
    }

    fn jmp(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.program_counter = addr;
    }

    fn jsr(&mut self, mode: &AddressingMode) {
        self.stack_push_u16(self.program_counter + 2 - 1);
        let addr = self.get_operand_address(mode);
        self.program_counter = addr;
    }

    fn rts(&mut self) {
        self.program_counter = self.stack_pop_u16() + 1;
    }

    fn rti(&mut self) {
        self.status = (self.stack_pop() & !BREAK_FLAG) | BREAK2_FLAG;
        self.program_counter = self.stack_pop_u16();
    }

    fn cmp(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        if self.register_a >= value {
            self.status = self.status | CARRY_FLAG;
        } else {
            self.status = self.status & !CARRY_FLAG;
        }

        self.update_zero_and_negative_flags(self.register_a.wrapping_sub(value));
    }

    fn cpx(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        if self.register_x >= value {
            self.status = self.status | CARRY_FLAG;
        } else {
            self.status = self.status & !CARRY_FLAG
        }

        self.update_zero_and_negative_flags(self.register_x.wrapping_sub(value));
    }

    fn cpy(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        if self.register_y >= value {
            self.status = self.status | CARRY_FLAG;
        } else {
            self.status = self.status & !CARRY_FLAG
        }

        self.update_zero_and_negative_flags(self.register_y.wrapping_sub(value));
    }

    fn dec(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        let value = value.wrapping_sub(1);
        self.mem_write(addr, value);
        self.update_zero_and_negative_flags(value);
    }

    fn dex(&mut self) {
        self.register_x = self.register_x.wrapping_sub(1);
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn dey(&mut self) {
        self.register_y = self.register_y.wrapping_sub(1);
        self.update_zero_and_negative_flags(self.register_y);
    }

    fn inc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        let value = value.wrapping_add(1);
        self.mem_write(addr, value);
        self.update_zero_and_negative_flags(value);
    }

    fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn iny(&mut self) {
        self.register_y = self.register_y.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_y);
    }

    fn clc(&mut self) {
        self.status = self.status & !CARRY_FLAG;
    }

    fn sec(&mut self) {
        self.status = self.status | CARRY_FLAG;
    }

    fn cld(&mut self) {
        self.status = self.status & !DECIMAL_FLAG;
    }

    fn sed(&mut self) {
        self.status = self.status | DECIMAL_FLAG;
    }

    fn cli(&mut self) {
        self.status = self.status & !INTERRUPT_DISABLE_FLAG;
    }

    fn sei(&mut self) {
        self.status = self.status | INTERRUPT_DISABLE_FLAG;
    }

    fn clv(&mut self) {
        self.status = self.status & !OVERFLOW_FLAG;
    }

    fn lda(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        self.register_a = value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn ldx(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        self.register_x = value;
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn ldy(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        self.register_y = value;
        self.update_zero_and_negative_flags(self.register_y);
    }

    fn sta(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.register_a);
    }

    fn stx(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.register_x);
    }

    fn sty(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.register_y);
    }

    fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn tay(&mut self) {
        self.register_y = self.register_a;
        self.update_zero_and_negative_flags(self.register_y)
    }

    fn tsx(&mut self) {
        self.register_x = self.stack_pointer;
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn txa(&mut self) {
        self.register_a = self.register_x;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn txs(&mut self) {
        self.stack_pointer = self.register_x;
    }

    fn tya(&mut self) {
        self.register_a = self.register_y;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn pha(&mut self) {
        self.stack_push(self.register_a);
    }

    fn pla(&mut self) {
        self.register_a = self.stack_pop();
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn php(&mut self) {
        self.stack_push((self.status | BREAK_FLAG) | BREAK2_FLAG);
    }

    fn plp(&mut self) {
        self.status = (self.stack_pop() & !BREAK_FLAG) | BREAK2_FLAG;
    }

    fn nop(&mut self) {
        // no operation
    }

    fn lax(&mut self, mode: &AddressingMode) {
        self.lda(mode);
        self.tax();
    }

    fn stack_push(&mut self, value: u8) {
        self.mem_write(0x100 + (self.stack_pointer as u16), value);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    }

    fn stack_pop(&mut self) -> u8 {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        self.mem_read(0x100 + (self.stack_pointer as u16))
    }

    fn stack_push_u16(&mut self, value: u16) {
        let hi = (value >> 8) as u8;
        let lo = (value & 0x00FF) as u8;
        self.stack_push(hi);
        self.stack_push(lo);
    }

    fn stack_pop_u16(&mut self) -> u16 {
        let lo = self.stack_pop() as u16;
        let hi = self.stack_pop() as u16;
        (hi << 8) | lo
    }

    fn update_zero_and_negative_flags(&mut self, result: u8) {
        if result == 0 {
            self.status = self.status | ZERO_FLAG;
        } else {
            self.status = self.status & !ZERO_FLAG;
        }

        if result & 0b1000_0000 != 0 {
            self.status = self.status | NEGATIVE_FLAG;
        } else {
            self.status = self.status & !NEGATIVE_FLAG;
        }
    }
}
