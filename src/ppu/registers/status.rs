pub struct StatusRegister {
    value: u8,
}

impl StatusRegister {
    const PPU_OPEN_BUS: u8 = 0b0001_1111;
    const SPRITE_OVERFLOW: u8 = 0b0010_0000;
    const SPRITE_ZERO_HIT: u8 = 0b0100_0000;
    const VBLANK_HAS_STARTED: u8 = 0b1000_0000;

    pub fn new() -> StatusRegister {
        StatusRegister { value: 0 }
    }

    pub fn set_sprite_zero_hit(&mut self, value: bool) {
        if value {
            self.value |= StatusRegister::SPRITE_ZERO_HIT;
        } else {
            self.value &= !StatusRegister::SPRITE_ZERO_HIT;
        }
    }

    pub fn is_in_vblank(&self) -> bool {
        self.value & StatusRegister::VBLANK_HAS_STARTED != 0
    }

    pub fn set_vblank_status(&mut self, value: bool) {
        if value {
            self.value |= StatusRegister::VBLANK_HAS_STARTED;
        } else {
            self.value &= !StatusRegister::VBLANK_HAS_STARTED;
        }
    }

    pub fn reset_vblank_status(&mut self) {
        self.set_vblank_status(false);
    }

    pub fn get(&self) -> u8 {
        self.value
    }
}
