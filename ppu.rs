struct PpuRegisters {
    _xscrl: u16,
    _yscrl: u16,
    _ly: u8,
}
pub struct PPU {
    pub _vram: [u8; 0xFFFF],
    _r : PpuRegisters,
    tile_data: [[[u8; 8]; 8]; 384], 
}
impl PPU {
pub fn new() -> PPU {
    PPU {
        _r: PpuRegisters {  
            _xscrl: 0,
            _yscrl: 0,
            _ly: 0,
        },
        _vram: [0; 0xFFFF],
        tile_data: [[[0; 8]; 8]; 384],
    }
}
pub fn update_tile(&mut self, mut addr: usize) -> (){
    addr &= 0x1FFEE;
    let mut saved_addr = addr;
    if addr&1 == 1 { saved_addr -= 1 }; // saved_addr will always be at High-Order byte
    let tile_index = (addr>>4)&511;
    let tile_line = (addr>>1)&7;
    let mut bit_index;
    let mut row = 0;
        while row < 8 {
            bit_index = 1 << (7-row);
            let result = (if self._vram[saved_addr]&(bit_index as u8) > 0 { 1 } else { 0 })+ (if self._vram[saved_addr+1]&(bit_index as u8) > 0 { 2 } else { 0 });
            self.tile_data[tile_index][tile_line][row] = result;
            row += 1;
        }
        println!("Updated tile_index: {}", tile_index);
    } 
}