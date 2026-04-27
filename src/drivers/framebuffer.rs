use crate::{bsp::bus_to_phys, drivers::mailbox::*};
use core::ptr::write_volatile;

pub const MBOX_REQUEST: u32 = 0;

// tags for setting settings
pub const TAG_SET_PHY_WH: u32 = 0x00048003;
pub const TAG_SET_VIRT_WH: u32 = 0x00048004;
pub const TAG_SET_DEPTH: u32 = 0x00048005;
pub const TAG_ALLOC_BUFFER: u32 = 0x00040001;
pub const TAG_GET_PITCH: u32 = 0x00040008;
pub const TAG_LAST: u32 = 0;

#[derive(Default)]
pub struct FrameBuffer {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub address: u32,
    pub pitch: u32,
    pub size: u32,
}

impl FrameBuffer {
    pub fn new(width: u32, height: u32, depth: u32) -> Self {
        Self {
            width,
            height,
            depth,
            ..Default::default()
        }
    }

    pub fn init(&mut self) -> Result<(), &'static str> {
        let mut mbox: MboxBuffer = MboxBuffer::new();
        mbox.buffer[0] = 35 * 4; // buffer size
        mbox.buffer[1] = MBOX_REQUEST; // 0 for this is a request

        // set physical height / width
        mbox.buffer[2] = TAG_SET_PHY_WH; // the tag
        mbox.buffer[3] = 8; // data length buffer (in bytes)
        mbox.buffer[4] = 8; // length of the allowed gpu response here
                            // data
        mbox.buffer[5] = self.width; // Width
        mbox.buffer[6] = self.height; // Height

        // virutal height
        mbox.buffer[7] = TAG_SET_VIRT_WH;
        mbox.buffer[8] = 8;
        mbox.buffer[9] = 8;
        mbox.buffer[10] = self.width;
        mbox.buffer[11] = self.height;

        // depth
        mbox.buffer[12] = TAG_SET_DEPTH;
        mbox.buffer[13] = 4;
        mbox.buffer[14] = 4;
        mbox.buffer[15] = self.depth; // 32 bits per pixel (RGB+Alpha)

        // allocate buffer
        mbox.buffer[16] = TAG_ALLOC_BUFFER;
        mbox.buffer[17] = 8;
        mbox.buffer[18] = 8;
        mbox.buffer[19] = 16; // Alignment requirement
        mbox.buffer[20] = 0; // The GPU will write the base address here

        // get pitch (bytes per line)
        mbox.buffer[21] = TAG_GET_PITCH;
        mbox.buffer[22] = 4;
        mbox.buffer[23] = 4;
        mbox.buffer[24] = 0; // GPU will write length here

        mbox.buffer[25] = TAG_LAST; // End tag

        // call the mailbox channel 8
        if mbox.call(MBOX_CHANNEL) {
            // if it successes read the data from the gpu
            let fb_address = bus_to_phys(mbox.buffer[19]);
            let fb_size = mbox.buffer[20];
            let pitch = mbox.buffer[24];

            self.pitch = pitch;
            self.address = fb_address;
            self.size = fb_size;

            // sends the frambuffer pointer, size and pitch
            Ok(())
        } else {
            Err("Failed to set up framebuffer via mailbox!")
        }
    }

    pub fn clear(&mut self) {
        for j in 0..self.height {
            for i in 0..self.width {
                self.draw_pixel(i, j, 0xFF000000);
            }
        }
    }
    pub fn clear_from(&mut self, line: u32) {
        for j in line..self.height {
            for i in 0..self.width {
                self.draw_pixel(i, j, 0xFF000000);
            }
        }
    }

    pub fn draw_pixel(&self, x: u32, y: u32, color: u32) {
        // 1024x768 is our max resolution
        if x >= 1024 || y >= 768 {
            return;
        }

        // Calculate the memory offset
        // y * pitch gives us the start of the row (in bytes)
        // x * 4 gives us the column offset (since each pixel is 4 bytes)
        let offset = (y * self.pitch) + (x * 4);

        unsafe {
            // Create a mutable raw pointer to the pixel's memory address
            let pixel_ptr = (self.address + offset) as *mut u32;
            // Volatile write ensures the compiler doesn't optimize this memory operation away
            write_volatile(pixel_ptr, color);
        }
    }

    pub fn draw_image(&self, w: usize, h: usize, data: &[u8]) {
        let mut data_idx = 0;

        for y in 0..h {
            for x in 0..w {
                let r = data[data_idx] as u32;
                let g = data[data_idx + 1] as u32;
                let b = data[data_idx + 2] as u32;
                data_idx += 3;

                let pixel_color = (r << 16) | (g << 8) | b;

                self.draw_pixel(
                    (self.width / 2) + x as u32 - (w / 2) as u32,
                    (self.height / 2) + y as u32 - (h / 2) as u32,
                    pixel_color,
                );
            }
        }
    }
}
