use crate::{cart::Rom, cpu::Mem, ppu::PPU};
use core::panic;

pub struct Bus {
    ram: [u8; 2048],
    prg_rom: Vec<u8>,
    ppu: PPU,
}

impl Bus {
    pub fn new(rom: Rom) -> Bus {
        let ppu = PPU::new(rom.chr_rom, rom.screen_mirroring);
        Bus {
            ram: [0; 2048],
            prg_rom: rom.prg_rom,
            ppu,
        }
    }

    fn read_prg_rom(&self, mut addr: u16) -> u8 {
        addr -= 0x8000;
        if self.prg_rom.len() == 0x4000 && addr >= 0x4000 {
            addr = addr % 0x4000;
        }
        self.prg_rom[addr as usize]
    }
}

impl Mem for Bus {
    fn mem_read(&mut self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x1FFF => {
                let mirror_down_addr = addr & 0b00000111_11111111;
                self.ram[mirror_down_addr as usize]
            }
            0x2000 | 0x2001 | 0x2003 | 0x2005 | 0x2006 => {
                panic!("Attempt to read from write-only PPU address {:X}", addr)
            }
            0x2002 => self.ppu.read_status(),
            0x2004 => {
                panic!("Attempt to read from OAM data")
            }
            0x2007 => self.ppu.read_data(),
            0x2008..=0x3FFF => {
                let mirror_down_addr = addr & 0b00100000_00000111;
                self.mem_read(mirror_down_addr)
            }
            0x8000..=0xFFFF => self.read_prg_rom(addr),
            _ => {
                println!("Ignoring mem access at {}", addr);
                0
            }
        }
    }

    fn mem_write(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..=0x1FFF => {
                let mirror_down_addr = addr & 0b11111111111;
                self.ram[mirror_down_addr as usize] = value;
            }
            0x2000 => {
                self.ppu.write_to_ctrl(value);
            }
            0x2001 => {
                self.ppu.write_to_mask(value);
            }
            0x2002 => {
                panic!("Attempt to write from read-only PPU address {:X}", addr)
            }
            0x2003 => {
                panic!("Attempt to write to OAM address register")
            }
            0x2004 => {
                panic!("Attempt to write to OAM data register")
            }
            0x2005 => {
                panic!("Attempt to write to PPU scroll register")
            }
            0x2006 => {
                self.ppu.write_to_ppu_addr(value);
            }
            0x2007 => {
                self.ppu.write_to_data(value);
            }
            0x2008..=0x3FFF => {
                let _mirror_down_addr = addr & 0b00100000_00000111;
                self.mem_write(addr, value);
            }
            0x8000..=0xFFFF => {
                panic!("Attempt to write to Cartridge ROM space")
            }
            _ => {
                println!("Ignoring mem write-access at {}", addr);
            }
        }
    }
}
