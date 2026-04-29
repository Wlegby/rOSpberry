use heapless::{String, Vec};
use ttf_parser::Face;

use crate::dbg;
use crate::drivers::framebuffer::FrameBuffer;
static FONT: &[u8] = include_bytes!("../font.otb");
pub const FONT_SIZE: (u32, u32) = (8, 16);

pub struct Console<'a> {
    lines: Vec<String<124>, 10>,
    x: u32,
    y: u32,
    font_face: Face<'a>,
    fb: FrameBuffer,
    pub color: u32,
}

impl<'a> Console<'a> {
    pub fn init(fb: FrameBuffer) -> Option<Console<'a>> {
        if let Ok(face) = Face::parse(FONT, 0) {
            return Some(Self {
                lines: Vec::new(),
                x: 0,
                y: 0,
                font_face: face.clone(),
                fb,
                color: 0xFFFFFFFF,
            });
        }
        None
    }

    pub fn print(&mut self, text: String<128>) {
        for line in text.lines() {
            let mut string = String::new();
            string.push_str(line);
            self.lines.push(string);
        }
        self.draw();
    }

    pub fn draw(&mut self) {
        self.y = 0;
        for line in self.lines.clone() {
            self.x = 0;
            for c in line.chars() {
                self.draw_char(c);
                self.x += 1;
            }
            self.y += 1;
        }
    }

    pub fn draw_char(&self, character: char) {
        let x_offset = self.x as u32 * FONT_SIZE.0;
        let y_offset = self.y as u32 * FONT_SIZE.1;

        if let Some(glyph_id) = self.font_face.glyph_index(character) {
            let strike_size = 16;

            // Get the bitmap data
            if let Some(raster_img) = self.font_face.glyph_raster_image(glyph_id, strike_size) {
                let bitmap_data = raster_img.data;

                // Safety check
                if raster_img.width != 8 || raster_img.height != 16 || bitmap_data.len() < 16 {
                    return;
                }

                // Loop through the 16 rows
                for y in 0..16 {
                    let row_byte = bitmap_data[y as usize];
                    if row_byte == 0 {
                        continue;
                    }

                    // Loop through the 8 columns
                    for x in 0..8 {
                        let bit_position = 7 - x;
                        let is_pixel_on = (row_byte >> bit_position) & 1 == 1;

                        if is_pixel_on {
                            self.fb.draw_pixel(x_offset + x, y_offset + y, self.color);
                        }
                    }
                }
            }
        }
    }
}
