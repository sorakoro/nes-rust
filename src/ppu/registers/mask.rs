pub struct MaskRegister {
    value: u8,
}

impl MaskRegister {
    const GRAYSCALE: u8 = 0b0000_0001;
    const SHOW_BACKGROUND_LEFTMOST_8: u8 = 0b0000_0010;
    const SHOW_SPRITES_LEFTMOST_8: u8 = 0b0000_0100;
    const SHOW_BACKGROUND: u8 = 0b0000_1000;
    const SHOW_SPRITES: u8 = 0b0001_0000;
    const EMPHASIZE_RED: u8 = 0b0010_0000;
    const EMPHASIZE_GREEN: u8 = 0b0100_0000;
    const EMPHASIZE_BLUE: u8 = 0b1000_0000;

    pub fn new() -> MaskRegister {
        MaskRegister { value: 0 }
    }

    pub fn update(&mut self, value: u8) {
        self.value = value;
    }
}
