use alloc::collections::vec_deque::VecDeque;
use alloc::string::{String, ToString};
use ttf_parser::Face;

use crate::dbg;
use crate::drivers::framebuffer::FrameBuffer;
static FONT: &[u8] = include_bytes!("../font.otb");
pub const FONT_SIZE: (u32, u32) = (8, 16);

pub struct Console<'a> {
    lines: VecDeque<String>,
    width: u32,
    height: u32,
    font_face: Face<'a>,
    x_pos: u32,
    y_pos: u32,
    fb: FrameBuffer,
    pub color: u32,
}

impl<'a> Console<'a> {
    pub fn init(fb: FrameBuffer) -> Option<Console<'a>> {
        if let Ok(face) = Face::parse(FONT, 0) {
            return Some(Self {
                lines: VecDeque::new(),
                width: fb.width / FONT_SIZE.0,
                height: fb.height / FONT_SIZE.1,
                font_face: face.clone(),
                x_pos: 0,
                y_pos: 0,
                fb,
                color: 0xFFFFFFFF,
            });
        }
        None
    }

    pub fn print(&mut self, text: &str) {
        for line in text.lines() {
            if self.lines.len() == self.height as usize {
                self.lines.pop_front();
                self.y_pos = 0;
                self.fb.clear();
                for line in self.lines.clone() {
                    self.draw_line(&line);
                    self.y_pos += 1;
                }
            }
            self.lines.push_back(line.to_string());
            self.draw_line(line);
            self.y_pos += 1;
        }
    }

    pub fn draw_line(&mut self, line: &str) {
        for c in line.chars() {
            self.draw_char(c);
            self.x_pos += 1;
        }
    }

    pub fn draw_char(&self, character: char) {
        let x_offset = self.x_pos * FONT_SIZE.0;
        let y_offset = self.y_pos as u32 * FONT_SIZE.1;
        // Get the Glyph ID
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

// use crate::drivers::framebuffer::FrameBuffer;
//
// use ttf_parser::Face;
//
// // Bake the font into the binary
// static FONT_DATA: &[u8] = include_bytes!("../font.otb");
//
// // 1. A top-level function to handle drawing text
// pub fn draw_string(fb: &FrameBuffer, text: &str, start_x: u32, start_y: u32, color: u32) {
//     // Parse the font exactly ONCE per string or per frame
//     if let Ok(face) = Face::parse(FONT_DATA, 0) {
//         let mut current_x = start_x;
//
//         for character in text.chars() {
//             // Pass the parsed 'face' by reference down to the drawing routine
//             draw_char_8x16(fb, &face, current_x, start_y, character, color);
//             current_x += 8; // Advance cursor right by 8 pixels for the next char
//         }
//     }
// }
//
// // 2. The drawing routine now accepts a reference to the Face
// pub fn draw_char_8x16(
//     fb: &FrameBuffer,
//     face: &Face<'_>, // <--- Passed by reference here!
//     x_offset: u32,
//     y_offset: u32,
//     character: char,
//     color_argb: u32,
// ) {
//     // Get the Glyph ID
//     if let Some(glyph_id) = face.glyph_index(character) {
//         let strike_size = 16;
//
//         // Get the bitmap data
//         if let Some(raster_img) = face.glyph_raster_image(glyph_id, strike_size) {
//             let bitmap_data = raster_img.data;
//
//             // Safety check
//             if raster_img.width != 8 || raster_img.height != 16 || bitmap_data.len() < 16 {
//                 return;
//             }
//
//             // Loop through the 16 rows
//             for y in 0..16 {
//                 let row_byte = bitmap_data[y as usize];
//                 if row_byte == 0 {
//                     continue;
//                 }
//
//                 // Loop through the 8 columns
//                 for x in 0..8 {
//                     let bit_position = 7 - x;
//                     let is_pixel_on = (row_byte >> bit_position) & 1 == 1;
//
//                     if is_pixel_on {
//                         fb.draw_pixel(x_offset + x, y_offset + y, color_argb);
//                     }
//                 }
//             }
//         }
//     }
// }
