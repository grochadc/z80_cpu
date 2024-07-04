extern crate minifb;
use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 160;
const HEIGHT: usize = 144;
const TILE_SET: [[[u8; 8]; 8]; 3] = [
    [
      [ 0, 0, 0, 0, 0, 0, 0, 0, ],
      [ 0, 0, 0, 0, 0, 0, 0, 0, ],
      [ 0, 0, 0, 0, 0, 0, 0, 0, ],
      [ 0, 0, 0, 0, 0, 0, 0, 0, ],
      [ 0, 0, 0, 0, 0, 0, 0, 0, ],
      [ 0, 0, 0, 0, 0, 0, 0, 0, ],
      [ 0, 0, 0, 0, 0, 0, 0, 0, ],
      [ 0, 0, 0, 0, 0, 0, 0, 0, ],
    ],
    [
      [ 1, 1, 3, 3, 3, 3, 1, 1, ],
      [ 1, 3, 2, 0, 0, 2, 3, 1, ],
      [ 3, 2, 0, 0, 0, 0, 2, 3, ],
      [ 3, 2, 0, 0, 0, 0, 2, 3, ],
      [ 3, 2, 2, 0, 0, 2, 2, 3, ],
      [ 3, 3, 2, 2, 2, 2, 3, 3, ],
      [ 1, 3, 3, 2, 2, 3, 3, 1, ],
      [ 1, 1, 3, 3, 3, 3, 1, 1, ],
    ],
    [
      [ 0, 0, 0, 0, 0, 0, 2, 1, ],
      [ 0, 0, 0, 0, 0, 2, 0, 2, ],
      [ 2, 2, 0, 0, 2, 1, 1, 2, ],
      [ 2, 0, 2, 0, 2, 1, 2, 1, ],
      [ 2, 0, 1, 2, 2, 2, 2, 0, ],
      [ 0, 2, 0, 1, 2, 2, 1, 0, ],
      [ 1, 1, 2, 1, 2, 2, 0, 0, ],
      [ 0, 1, 1, 2, 2, 1, 0, 2, ],
    ],
  ];
fn main() {
    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e)
    });

    window.set_target_fps(60);

    fn render_scanline(tile_line: usize, local_buffer: &mut Vec<u32>) -> () {
        // increase tile_line once per scanline

        let palette = [
            0xFFFFFF, // color 0
            0xC0C0C0, // color 1
            0x5C5C5C, // color 2
            0x000000, // color 3
        ];

        let mut row = 0;
        let mut pixel: u32;
        for i in 0..160 {
            local_buffer[i] = palette[TILE_SET[1][tile_line][row] as usize];

            row += 1;
            if row == 8 {
                row = 0;
            }
        }
    }

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut tile_line_ = 0; 
    for i in 0..144 {
        if tile_line_ == 7 { tile_line_ = 0 };
        render_scanline(tile_line_, &mut buffer);
        tile_line_ += 1;
    }

    while window.is_open() && !window.is_key_down(Key::Escape) {
        /*
        for (i, pixel) in buffer.iter_mut().enumerate() {
            let color = i & 3;
            *pixel = palette[color as usize];
        }
         */
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}