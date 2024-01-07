use std::collections::HashMap;

use crate::{
    cpu::{AddressingMode, Mem, CPU},
    opcodes,
};

#[allow(dead_code)]
pub fn trace(cpu: &mut CPU) -> String {
    let ref opscodes: HashMap<u8, &'static opcodes::OpCode> = *opcodes::OPCODES_MAP;

    let code = cpu.mem_read(cpu.program_counter);
    let ops = opscodes.get(&code).unwrap();

    let begin = cpu.program_counter;
    let mut hex_dump = vec![];
    hex_dump.push(code);

    let (mem_addr, stored_value) = match ops.mode {
        AddressingMode::Implied | AddressingMode::Accumulator => (0, 0),
        _ => {
            let addr = cpu.get_absolute_address(&ops.mode, begin + 1);
            (addr, cpu.mem_read(addr))
        }
    };

    let tmp = match ops.length {
        1 => match ops.code {
            0x0a | 0x4a | 0x2a | 0x6a => {
                format!("A ")
            }
            _ => {
                format!("")
            }
        },
        2 => {
            let addr = cpu.mem_read(begin + 1);
            hex_dump.push(addr);

            match ops.mode {
                AddressingMode::Immediate => {
                    format!("#${:02x}", addr)
                }
                AddressingMode::ZeroPage => {
                    format!("${:02x} = {:02x}", mem_addr, stored_value)
                }
                AddressingMode::ZeroPage_X => {
                    format!("${:02x},X @ {:02x} = {:02x}", addr, mem_addr, stored_value)
                }
                AddressingMode::ZeroPage_Y => {
                    format!("${:02x},Y @ {:02x} = {:02x}", addr, mem_addr, stored_value)
                }
                AddressingMode::RELATIVE => {
                    format!("${:04x}", mem_addr)
                }
                AddressingMode::Indirect_X => {
                    format!(
                        "(${:02x},X) @ {:02x} = {:04x} = {:02x} ",
                        addr,
                        addr.wrapping_add(cpu.register_x),
                        mem_addr,
                        stored_value
                    )
                }
                AddressingMode::Indirect_Y => {
                    format!(
                        "(${:02x}),Y = {:04x} @ {:04x} = {:02x}",
                        addr,
                        mem_addr.wrapping_sub(cpu.register_y as u16),
                        mem_addr,
                        stored_value
                    )
                }
                _ => {
                    panic!(
                        "unexpected addressing mode {:?} has ops-len 2. code {:02x}",
                        ops.mode, ops.code
                    )
                }
            }
        }
        3 => {
            let lo = cpu.mem_read(begin + 1);
            let hi = cpu.mem_read(begin + 2);
            hex_dump.push(lo);
            hex_dump.push(hi);

            let addr = cpu.mem_read_u16(begin + 1);

            match ops.mode {
                AddressingMode::Absolute => match ops.code {
                    0x4c | 0x20 => {
                        format!("${:04x}", addr)
                    }
                    _ => {
                        format!("${:04x} = {:02x}", mem_addr, stored_value)
                    }
                },
                AddressingMode::Absolute_X => {
                    format!("${:04x},X @ {:04x} = {:02x}", addr, mem_addr, stored_value)
                }
                AddressingMode::Absolute_Y => {
                    format!("${:04x},Y @ {:04x} = {:02x}", addr, mem_addr, stored_value)
                }
                AddressingMode::Indirect => {
                    format!("(${:04x}) = {:04x}", addr, mem_addr)
                }
                _ => {
                    panic!(
                        "unexpected addressing mode {:?} has ops-len 3. code {:02x}",
                        ops.mode, ops.code
                    )
                }
            }
        }
        _ => String::from(""),
    };

    let hex_str = hex_dump
        .iter()
        .map(|z| format!("{:02x}", z))
        .collect::<Vec<String>>()
        .join(" ");

    let asm_str = format!("{:04x}  {:8} {: >4} {}", begin, hex_str, ops.mnemonic, tmp)
        .trim()
        .to_string();

    format!(
        "{:47} A:{:02x} X:{:02x} Y:{:02x} P:{:02x} SP:{:02x}",
        asm_str, cpu.register_a, cpu.register_x, cpu.register_y, cpu.status, cpu.stack_pointer
    )
    .to_ascii_uppercase()
}
