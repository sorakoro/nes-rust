use crate::cpu::AddressingMode;
use once_cell::sync::Lazy;
use std::collections::HashMap;

pub struct OpCode {
    pub code: u8,
    pub mnemonic: &'static str,
    pub length: u8,
    pub cycles: u8,
    pub mode: AddressingMode,
}

impl OpCode {
    fn new(code: u8, mnemonic: &'static str, length: u8, cycles: u8, mode: AddressingMode) -> Self {
        OpCode {
            code,
            mnemonic,
            length,
            cycles,
            mode,
        }
    }
}

static OPCODES: Lazy<Vec<OpCode>> = Lazy::new(|| {
    vec![
        // 演算
        OpCode::new(0x69, "ADC", 2, 2, AddressingMode::Immediate),
        OpCode::new(0x65, "ADC", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x75, "ADC", 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new(0x6d, "ADC", 3, 4, AddressingMode::Absolute),
        OpCode::new(0x7d, "ADC", 3, 4, AddressingMode::Absolute_X),
        OpCode::new(0x79, "ADC", 3, 4, AddressingMode::Absolute_Y),
        OpCode::new(0x61, "ADC", 2, 6, AddressingMode::Indirect_X),
        OpCode::new(0x71, "ADC", 2, 5, AddressingMode::Indirect_Y),
        OpCode::new(0xe9, "SBC", 2, 2, AddressingMode::Immediate),
        OpCode::new(0xe5, "SBC", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xf5, "SBC", 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new(0xed, "SBC", 3, 4, AddressingMode::Absolute),
        OpCode::new(0xfd, "SBC", 3, 4, AddressingMode::Absolute_X),
        OpCode::new(0xf9, "SBC", 3, 4, AddressingMode::Absolute_Y),
        OpCode::new(0xe1, "SBC", 2, 6, AddressingMode::Indirect_X),
        OpCode::new(0xf1, "SBC", 2, 5, AddressingMode::Indirect_Y),
        // 論理演算
        OpCode::new(0x29, "AND", 2, 2, AddressingMode::Immediate),
        OpCode::new(0x25, "AND", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x35, "AND", 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new(0x2d, "AND", 3, 4, AddressingMode::Absolute),
        OpCode::new(0x3d, "AND", 3, 4, AddressingMode::Absolute_X),
        OpCode::new(0x39, "AND", 3, 4, AddressingMode::Absolute_Y),
        OpCode::new(0x21, "AND", 2, 6, AddressingMode::Indirect_X),
        OpCode::new(0x31, "AND", 2, 5, AddressingMode::Indirect_Y),
        OpCode::new(0x49, "EOR", 2, 2, AddressingMode::Immediate),
        OpCode::new(0x45, "EOR", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x55, "EOR", 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new(0x4d, "EOR", 3, 4, AddressingMode::Absolute),
        OpCode::new(0x5d, "EOR", 3, 4, AddressingMode::Absolute_X),
        OpCode::new(0x59, "EOR", 3, 4, AddressingMode::Absolute_Y),
        OpCode::new(0x41, "EOR", 2, 6, AddressingMode::Indirect_X),
        OpCode::new(0x51, "EOR", 2, 5, AddressingMode::Indirect_Y),
        OpCode::new(0x09, "ORA", 2, 2, AddressingMode::Immediate),
        OpCode::new(0x05, "ORA", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x15, "ORA", 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new(0x0d, "ORA", 3, 4, AddressingMode::Absolute),
        OpCode::new(0x1d, "ORA", 3, 4, AddressingMode::Absolute_X),
        OpCode::new(0x19, "ORA", 3, 4, AddressingMode::Absolute_Y),
        OpCode::new(0x01, "ORA", 2, 6, AddressingMode::Indirect_X),
        OpCode::new(0x11, "ORA", 2, 5, AddressingMode::Indirect_Y),
    ]
});

pub static OPCODES_MAP: Lazy<HashMap<u8, &'static OpCode>> = Lazy::new(|| {
    let mut map = HashMap::new();
    for opcode in &*OPCODES {
        map.insert(opcode.code, opcode);
    }
    map
});
