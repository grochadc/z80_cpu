use crate::ppu::PPU;

pub struct MMU {
    ppu: PPU,
    _inbios: u8,
    _bios: [u8; 11],
    _rom: [u8; 10],
}

impl MMU {
    pub fn new(rom: [u8; 10]) -> MMU {
        return MMU {
            ppu: PPU::new(),
            _inbios:1,
            _bios: [
                0x21, 0x80, 0x00, 0x0E, 0x04, 0x36, 0x0A, 0x0D, 0x20, 0xFB, 0x01,
                ],
            _rom: rom,
        }
    }
    pub fn rb(&mut self, addr: usize, pc: usize) -> u8 {
        let masked_addr = addr & 0xF000;
        println!("RB Matching masked addr {:#2x} Addr{:#2x}",masked_addr, addr);
        match masked_addr {
            0x0000 => {
                if self._inbios == 1 {
                    if addr<0x0100 { return self._bios[addr] }
                    else if pc==0x0100 { self._inbios = 0; println!("Leaving bios"); return self._bios[addr-1]; };
                    return 0;
                } else {
                    return self._rom[addr];
                }
            },
            0x8000 | 0x9000 => {
                println!("Reading vram");
                return self.ppu._vram[addr];
            },
            _ => {
                return 0;
            }
        }
    }
    pub fn wb(&mut self, addr: usize, val: u8) -> usize {
        let masked_addr = addr & 0xF000;
        println!("WB Matching masked addr {:#2x} Addr{:#2x}",masked_addr, addr);
        match masked_addr {
            0x0000 => {
                if self._inbios == 1 { self._bios[addr] = val; return addr; }
                else { self._rom[addr] = val; return addr; }
            },
            0x8000 | 0x9000 => {
                println!("Writing vram");
                self.ppu._vram[addr & 0xF000] = val;
                self.ppu.update_tile(addr);
                return addr;
            },
            _ =>  { println!("Address not found"); return addr; },
        }
    }
    pub fn rw(&mut self, addr: usize, pc: usize) -> u16 {
        return ((self.rb(addr, pc) as u16) << 8) | self.rb(addr+1, pc) as u16;
    }
}