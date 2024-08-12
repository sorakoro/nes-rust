use registers::mask::MaskRegister;

use self::registers::{address::AddrRegister, control::ControlRegister};
use crate::cart::Mirroring;

pub mod registers;

pub struct PPU {
    pub chr_rom: Vec<u8>,
    pub palette_table: [u8; 32],
    pub vram: [u8; 2048],
    pub oam_data: [u8; 256],
    pub mirroring: Mirroring,
    pub addr: AddrRegister,
    pub ctrl: ControlRegister,
    pub mask: MaskRegister,
    internal_data_buf: u8,
}

impl PPU {
    pub fn new(chr_rom: Vec<u8>, mirroring: Mirroring) -> Self {
        PPU {
            chr_rom,
            palette_table: [0; 32],
            vram: [0; 2048],
            oam_data: [0; 64 * 4],
            mirroring,
            addr: AddrRegister::new(),
            ctrl: ControlRegister::new(),
            mask: MaskRegister::new(),
            internal_data_buf: 0,
        }
    }

    pub fn write_to_ppu_addr(&mut self, value: u8) {
        self.addr.update(value);
    }

    pub fn write_to_ctrl(&mut self, value: u8) {
        self.ctrl.update(value);
    }

    pub fn write_to_mask(&mut self, value: u8) {
        self.mask.update(value);
    }

    pub fn read_data(&mut self) -> u8 {
        let addr = self.addr.get();
        self.increment_vram_addr();

        match addr {
            0x0000..=0x1FFF => {
                let result = self.internal_data_buf;
                self.internal_data_buf = self.chr_rom[addr as usize];
                result
            }
            0x2000..=0x3EFF => {
                let result = self.internal_data_buf;
                self.internal_data_buf = self.vram[self.mirror_vram_addr(addr) as usize];
                result
            }
            0x3F00..=0x3FFF => {
                let result = self.internal_data_buf;
                self.internal_data_buf =
                    self.palette_table[self.mirror_palette_addr(addr) as usize];
                result
            }
            _ => panic!("unexpected access to mirrored space {}", addr),
        }
    }

    pub fn write_to_data(&mut self, value: u8) {
        let addr = self.addr.get();
        self.increment_vram_addr();

        match addr {
            0x0000..=0x1FFF => {
                panic!(
                    "addr space 0x0000..0x1FFF is not expected to be used, requested = {}",
                    addr
                )
            }
            0x2000..=0x3EFF => {
                self.vram[self.mirror_vram_addr(addr) as usize] = value;
            }
            0x3F00..=0x3FFF => {
                self.palette_table[self.mirror_palette_addr(addr) as usize] = value;
            }
            _ => panic!("unexpected access to mirrored space {}", addr),
        }
    }

    fn increment_vram_addr(&mut self) {
        self.addr.increment(self.ctrl.vram_addr_increment());
    }

    fn mirror_vram_addr(&mut self, addr: u16) -> u16 {
        let mirrored_vram_addr = addr & 0b10_1111_1111_1111;
        let vram_index = mirrored_vram_addr - 0x2000;
        let nametable = vram_index / 0x400;
        match (&self.mirroring, nametable) {
            (Mirroring::VERTICAL, 2) => vram_index - 0x800,
            (Mirroring::VERTICAL, 3) => vram_index - 0x800,
            (Mirroring::HORIZONTAL, 1) => vram_index - 0x400,
            (Mirroring::HORIZONTAL, 2) => vram_index - 0x400,
            (Mirroring::HORIZONTAL, 3) => vram_index - 0x800,
            _ => vram_index,
        }
    }

    fn mirror_palette_addr(&mut self, addr: u16) -> u16 {
        let mirrored_palette_addr = addr & 0b0000_0000_0001_1111;
        match mirrored_palette_addr {
            0x10 => 0x00,
            0x14 => 0x04,
            0x18 => 0x08,
            0x1C => 0x0C,
            _ => mirrored_palette_addr,
        }
    }
}
