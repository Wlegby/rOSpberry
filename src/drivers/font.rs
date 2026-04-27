use crate::drivers::framebuffer::FrameBuffer;

use ttf_parser::Face;

// Bake the font into the binary
static FONT_DATA: &[u8] = include_bytes!("../font.otb");

// 1. A top-level function to handle drawing text
pub fn draw_string(fb: &FrameBuffer, text: &str, start_x: u32, start_y: u32, color: u32) {
    // Parse the font exactly ONCE per string or per frame
    if let Ok(face) = Face::parse(FONT_DATA, 0) {
        let mut current_x = start_x;

        for character in text.chars() {
            // Pass the parsed 'face' by reference down to the drawing routine
            draw_char_8x16(fb, &face, current_x, start_y, character, color);
            current_x += 8; // Advance cursor right by 8 pixels for the next char
        }
    }
}

// 2. The drawing routine now accepts a reference to the Face
pub fn draw_char_8x16(
    fb: &FrameBuffer,
    face: &Face<'_>, // <--- Passed by reference here!
    x_offset: u32,
    y_offset: u32,
    character: char,
    color_argb: u32,
) {
    // Get the Glyph ID
    if let Some(glyph_id) = face.glyph_index(character) {
        let strike_size = 16;

        // Get the bitmap data
        if let Some(raster_img) = face.glyph_raster_image(glyph_id, strike_size) {
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
                        fb.draw_pixel(x_offset + x, y_offset + y, color_argb);
                    }
                }
            }
        }
    }
}
