mod ppu;
mod mmu;

use mmu::MMU;

struct Registers {
    a: u8,
    b: u8,
    c: u8,
    h: u8,
    l: u8,
    pc: usize,
    m: u8,
    f: u8,
}

pub struct Z80 {
    _r: Registers,
    _halt: i32,
    mmu: MMU,
}


impl Z80 {
    pub fn new(rom: [u8; 10]) -> Z80 {
        Z80 {
            mmu: MMU::new(rom),
            _r: Registers {
                a:0,
                b:0,
                c: 0,
                h:0,
                l:0,
                pc: 0,
                m: 0,
                f: 0,
            },
            _halt: 0,
        }
    }
    pub fn start(&mut self) -> () {
        while self._halt < 1 {
            self.exec();
        }
    }
    fn exec(&mut self) -> () {
        let opcode = MMU::rb(&mut self.mmu, self._r.pc, self._r.pc);
        println!("Executing instruction {:#2x} from PC {:#2x}", opcode, self._r.pc);
        self._r.pc += 1;
        match opcode {
            0 => self.nop(),
            1 => self.halt(),
            2 => self.ldamm(),
            0x0A => self.ldabcm(),
            0x0E => self.ldc_n(),
            0x0D => self.decr_c(),
            0x20 => self.jrnz_n(),
            0x21 => self.ldhlnn(),
            0x22 => self.ldhli_a(),
            0x36 => self.ldhlmn(),
            _ => self.instr_not_found(opcode),
        }
    }
    fn nop(&mut self) -> () { self._r.m = 1; }
    fn ldamm(&mut self) -> () { let addr = MMU::rw(&mut self.mmu, self._r.pc, self._r.pc); self._r.pc += 2; self._r.a = MMU::rb(&mut self.mmu, addr as usize, self._r.pc); self._r.m = 4; }
    fn ldc_n(&mut self) -> () { 
        self._r.c = MMU::rb(&mut self.mmu, self._r.pc, self._r.pc); 
        println!("Loading c with value {:#2x}", self._r.c);
        self._r.pc += 1; self._r.m=2; }
    fn ldabcm(&mut self) -> () { self._r.a = MMU::rb(&mut self.mmu, (((self._r.b as u16) << 8) | self._r.c as u16) as usize, self._r.pc) }
    fn ldhlnn(&mut self) -> () { self._r.h = MMU::rb(&mut self.mmu, self._r.pc, self._r.pc); self._r.l = MMU::rb(&mut self.mmu, self._r.pc+1, self._r.pc); self._r.pc += 2; self._r.m = 3; println!("LD(HL)n H {:#2x} L {:#2x}", self._r.h, self._r.l)}
    fn ldhli_a(&mut self) -> () {
        let hbyte = self._r.h as u16;
        let dest_addr = (hbyte*256)+self._r.l as u16;
        MMU::wb(&mut self.mmu, dest_addr as usize, self._r.a); 
        self._r.l = (self._r.l+1)&255;
        if self._r.l == 0 { self._r.h = (self._r.h+1)&255; } 
    }
    fn ldhlmn(&mut self) -> () { let addr = ((self._r.h as u16) << 8) | self._r.l as u16; let val = MMU::rb(&mut self.mmu,self._r.pc, self._r.pc); MMU::wb(&mut self.mmu, addr as usize, val); self._r.pc += 1; self._r.m=1; }
    fn jrnz_n(&mut self) -> () {
        let mut i = MMU::rb(&mut self.mmu, self._r.pc, self._r.pc) as isize;
        if i > 127 {  i=-((!i+1)&255); };
        self._r.pc += 1;
        self._r.m = 2;
        if (self._r.f&0x80) == 0 { 
            self._r.pc = self._r.pc.checked_add_signed(i).expect("REASON"); 
            self._r.m += 1; 
        }
    }
    fn decr_c(&mut self) -> () { self._r.c -= 1; self._r.c&=255; 
        println!("c register after decr_r {:#2x}", self._r.c);
        if self._r.c == 0 { self._r.f=0x80 } else { self._r.f=0 }; self._r.m=1;
        println!("f register after decr_c {:#2X}", self._r.f);
    }
    fn halt(&mut self) -> () { self._halt = 1; self._r.m = 1; println!("Set halt flag"); }
    fn instr_not_found(&mut self, opcode: u8) -> () { println!("Couldn't find instruction with opcode {}", opcode); self.halt(); }
}

fn main() {
    let rom: [u8; 10] = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    let _tiles_for_vram = [
        0x03, 0x03, 0x0D, 0x0E, 0x14, 0x18, 0x20, 0x30, 0x30, 0x20, 0x2C, 0x30, 0x2B, 0x34, 0x68, 0x36,
        0xE0, 0xE0, 0xD8, 0x38, 0x14, 0x0C, 0x02, 0x06, 0x06, 0x02, 0x1A, 0x06, 0xEA, 0x16, 0x0A, 0x36, 
    ];
    let mut cpu = Z80::new(rom);
    cpu.start();
}