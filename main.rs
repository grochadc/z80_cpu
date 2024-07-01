fn main() {
    pub struct MMU {
        _inbios: i32,
        _bios: [i32; 11],
    }

    impl MMU {
        fn new(program: [i32; 11]) -> MMU {
            MMU {
                _inbios:1,
                _bios: program,
            }
        }
        fn rb(&self, addr: usize) -> i32 {
            if addr < 20 {
                return self._bios[addr];
            }
            else {
                return 0
            }
        }
        fn wb(&mut self, addr: usize, val: i32) -> usize {
            if self._inbios == 0 {
                self._bios[addr] = val;
                return addr;
            }
            else { return 0 }
        }
        fn rw(&mut self, addr: usize) -> i32 {
            return self.rb(addr) + (self.rb(addr+1)<<8)
        }
    }
    
    
    struct Registers {
        a: i32,
        b: i32,
        c: i32,
        h: i32,
        l: i32,
        pc: usize,
        m: i32, 
    }

    pub struct Z80 {
        _r: Registers,
        _halt: i32,
        mmu: MMU,
    }
    

    impl Z80 {
        pub fn new(mmu: MMU) -> Z80 {
            Z80 {
                mmu,
                _r: Registers {
                    a:0,
                    b:0,
                    c: 0,
                    h:0,
                    l:0,
                    pc: 0,
                    m: 0,
                },
                _halt: 0,
            }
        }
        fn exec(&mut self) -> () {
            let opcode = MMU::rb(&self.mmu, self._r.pc);
            println!("Executing instruction {:#2x} from PC {:#2x}", opcode, self._r.pc);
            self._r.pc += 1;
            match opcode {
                0 => self.nop(),
                1 => self.halt(),
                2 => self.ldamm(),
                0x21 => self.ldhlnn(),
                _ => self.instr_not_found(opcode),
            }
        }
        fn nop(&mut self) -> () { self._r.m = 1; }
        fn ldamm(&mut self) -> () { let addr = MMU::rw(&mut self.mmu, self._r.pc); self._r.pc += 2; self._r.a = MMU::rb(&self.mmu, addr as usize); self._r.m = 4; }
        fn ldabcm(&mut self) -> () { self._r.a = MMU::rb(&mut self.mmu, ((self._r.b)+self._r.c<<8) as usize) }
        fn ldhlnn(&mut self) -> () { self._r.h = MMU::rb(&mut self.mmu, self._r.pc); self._r.l = MMU::rb(&mut self.mmu, self._r.pc+1); self._r.pc += 2; self._r.m = 3; }
        fn halt(&mut self) -> () { self._halt = 1; self._r.m = 1; println!("Set halt flag"); }
        fn instr_not_found(&mut self, opcode: i32) -> () { println!("Couldn't find instruction with opcode {}", opcode); self.halt(); }
    }
    
    let bios: [i32; 11] = [
        0x00, 0x21, 0x05, 0x01, 0x01, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F
    ];
    let mut cpu = Z80::new(MMU::new(bios));
    
    while cpu._halt < 1 {
        cpu.exec();
    }
}
