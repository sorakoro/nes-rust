pub struct ControlRegister {
    value: u8,
}

impl ControlRegister {
    // const NAMETABLE1: u8 = 0b00000001;
    // const NAMETABLE2: u8 = 0b00000010;
    const VRAM_ADD_INCREMENT: u8 = 0b00000100;
    const SPRITE_PATTERN_ADDR: u8 = 0b00001000;
    const BACKROUND_PATTERN_ADDR: u8 = 0b00010000;
    const SPRITE_SIZE: u8 = 0b00100000;
    const MASTER_SLAVE_SELECT: u8 = 0b01000000;
    const GENERATE_NMI: u8 = 0b10000000;

    pub fn new() -> Self {
        ControlRegister { value: 0 }
    }

    pub fn nametable_addr(&self) -> u16 {
        match self.value & 0b11 {
            0 => 0x2000,
            1 => 0x2400,
            2 => 0x2800,
            3 => 0x2C00,
            _ => panic!("not possible"),
        }
    }

    pub fn vram_addr_increment(&self) -> u8 {
        if (self.value & ControlRegister::VRAM_ADD_INCREMENT) != 0 {
            32
        } else {
            1
        }
    }

    pub fn sprt_pattern_addr(&self) -> u16 {
        if (self.value & ControlRegister::SPRITE_PATTERN_ADDR) != 0 {
            0x1000
        } else {
            0
        }
    }

    pub fn bknd_pattern_addr(&self) -> u16 {
        if (self.value & ControlRegister::BACKROUND_PATTERN_ADDR) != 0 {
            0x1000
        } else {
            0
        }
    }

    pub fn sprite_size(&self) -> u8 {
        if (self.value & ControlRegister::SPRITE_SIZE) != 0 {
            16
        } else {
            8
        }
    }

    pub fn master_slave_select(&self) -> u8 {
        if (self.value & ControlRegister::MASTER_SLAVE_SELECT) != 0 {
            1
        } else {
            0
        }
    }

    pub fn generate_vblank_nmi(&self) -> bool {
        return (self.value & ControlRegister::GENERATE_NMI) != 0;
    }

    pub fn update(&mut self, value: u8) {
        self.value = value
    }
}
