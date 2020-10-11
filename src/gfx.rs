use crate::bmp::*;
use crate::winapi::*;
use std::ops::*;

pub type Color = [u8; 3];
pub type Palette = Vec<Color>;

const BIT_COUNT: i32 = 32;
const BYTES_PER_PIXEL: i32 = BIT_COUNT / 8;

pub struct BackBuffer {
    pub width: i32,
    pub height: i32,
    pub info: BitmapInfo,
    pub memory: Vec<u8>,
}

pub struct SpriteSheet {
    pub bitmap: Bitmap,
    pub sprite_width: i32,
    pub sprite_height: i32,
    pub sprites_per_row: i32,
}

impl Index<usize> for BackBuffer {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        return &self.memory[index];
    }
}

impl IndexMut<usize> for BackBuffer {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return &mut self.memory[index];
    }
}

impl SpriteSheet {
    pub fn new(bitmap: Bitmap, sprite_width: i32, sprite_height: i32) -> Self {
        return SpriteSheet { sprites_per_row: bitmap.width / sprite_width, sprite_width: sprite_width, sprite_height: sprite_height, bitmap: bitmap };
    }
}

impl BackBuffer {
    pub fn new(width: i32, height: i32) -> BackBuffer {
        return BackBuffer {
            width: width,
            height: height,
            info: BitmapInfo {
                header: BitmapInfoHeader {
                    size: std::mem::size_of::<BitmapInfoHeader>() as u32,
                    width: width,
                    height: -height,
                    planes: 1,
                    bit_count: BIT_COUNT as u16,
                    compression: BI_RGB,
                    size_image: 0,
                    x_pels_per_meter: 0,
                    y_pels_per_meter: 0,
                    colors_used: 0,
                    colors_important: 0,
                },
                colors: 0,
            },
            memory: vec![0; (width * height * BYTES_PER_PIXEL) as usize],
        };
    }

    pub fn clear(&mut self, color: Color) {
        for i in 0..self.width * self.height {
            let index = (i * BYTES_PER_PIXEL) as usize;
            self[index + 0] = color[2];
            self[index + 1] = color[1];
            self[index + 2] = color[0];
        }
    }

    pub fn draw_sprite(&mut self, palette: &Palette, spritesheet: &SpriteSheet, x_dest: i32, y_dest: i32, index: i32) {
        self.draw_subsprite(palette, spritesheet, x_dest, y_dest, index, 0, 0, spritesheet.sprite_width, spritesheet.sprite_height);
    }

    pub fn draw_subsprite(&mut self, palette: &Palette, spritesheet: &SpriteSheet, x_dest: i32, y_dest: i32, index: i32, x_src_offs: i32, y_src_offs: i32, width: i32, height: i32) {
        self.draw_subbitmap(
            palette,
            &spritesheet.bitmap,
            x_dest,
            y_dest,
            (index % spritesheet.sprites_per_row) * spritesheet.sprite_width + x_src_offs,
            (index / spritesheet.sprites_per_row) * spritesheet.sprite_height + y_src_offs,
            width,
            height,
        );
    }

    pub fn draw_text(&mut self, palette: &Palette, font: &SpriteSheet, text: &str, x_dest: i32, y_dest: i32) {
        let mut x = x_dest;

        for character in text.chars() {
            if character != ' ' {
                self.draw_sprite(palette, font, x, y_dest, character as i32 - 'A' as i32);
            }

            x += font.sprite_width;
        }
    }

    pub fn draw_subbitmap(&mut self, palette: &Palette, bitmap: &Bitmap, x_dest: i32, y_dest: i32, x_src: i32, y_src: i32, width: i32, height: i32) {
        let mut src_row = (x_src + y_src * bitmap.width) as usize;
        let mut dest_row = ((x_dest + y_dest * self.width) * BYTES_PER_PIXEL) as usize;
        for _y in 0..height {
            let mut src = src_row;
            let mut dest = dest_row;

            for _x in 0..width {
                let palette_index = bitmap[src] as usize;
                src += 1;
                self[dest] = palette[palette_index][2];
                dest += 1;
                self[dest] = palette[palette_index][1];
                dest += 1;
                self[dest] = palette[palette_index][0];
                dest += 2;
            }

            src_row += bitmap.width as usize;
            dest_row += (self.width * BYTES_PER_PIXEL) as usize;
        }
    }
}
