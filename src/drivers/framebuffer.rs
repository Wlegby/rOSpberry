use crate::drivers::mailbox::*;
use core::ptr::write_volatile;

#[derive(Default)]
pub struct FrameBufferSettings {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub address: u32,
    pub pitch: u32,
    pub size: u32,
}

pub const MBOX_REQUEST: u32 = 0;

// tags for setting settings
pub const TAG_SET_PHY_WH: u32 = 0x00048003;
pub const TAG_SET_VIRT_WH: u32 = 0x00048004;
pub const TAG_SET_DEPTH: u32 = 0x00048005;
pub const TAG_ALLOC_BUFFER: u32 = 0x00040001;
pub const TAG_GET_PITCH: u32 = 0x00040008;
pub const TAG_LAST: u32 = 0;

pub fn init_framebuffer(settings: &mut FrameBufferSettings) -> Result<(), &'static str> {
    let mut mbox: MboxBuffer = MboxBuffer::new();
    mbox.buffer[0] = 35 * 4; // buffer size
    mbox.buffer[1] = MBOX_REQUEST; // 0 for this is a request

    // set physical height / width
    mbox.buffer[2] = TAG_SET_PHY_WH; // the tag
    mbox.buffer[3] = 8; // data length buffer (in bytes)
    mbox.buffer[4] = 8; // length of the allowed gpu response here
                        // data
    mbox.buffer[5] = settings.width; // Width
    mbox.buffer[6] = settings.height; // Height

    // virutal height
    mbox.buffer[7] = TAG_SET_VIRT_WH;
    mbox.buffer[8] = 8;
    mbox.buffer[9] = 8;
    mbox.buffer[10] = settings.width;
    mbox.buffer[11] = settings.height;

    // depth
    mbox.buffer[12] = TAG_SET_DEPTH;
    mbox.buffer[13] = 4;
    mbox.buffer[14] = 4;
    mbox.buffer[15] = settings.depth; // 32 bits per pixel (RGB+Alpha)

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
        let fb_address = mbox.buffer[19] & 0x3FFF_FFFF; // Convert GPU address to CPU bus address
        let fb_size = mbox.buffer[20];
        let pitch = mbox.buffer[24];

        settings.pitch = pitch;
        settings.address = fb_address;
        settings.size = fb_size;

        // sends the frambuffer pointer, size and pitch
        Ok(())
    } else {
        Err("Failed to set up framebuffer via mailbox!")
    }
}

pub fn draw_pixel(fb_addr: u32, pitch: u32, x: u32, y: u32, color: u32) {
    // 1024x768 is our max resolution
    if x >= 1024 || y >= 768 {
        return;
    }

    // Calculate the memory offset
    // y * pitch gives us the start of the row (in bytes)
    // x * 4 gives us the column offset (since each pixel is 4 bytes)
    let offset = (y * pitch) + (x * 4);

    unsafe {
        // Create a mutable raw pointer to the pixel's memory address
        let pixel_ptr = (fb_addr + offset) as *mut u32;
        // Volatile write ensures the compiler doesn't optimize this memory operation away
        write_volatile(pixel_ptr, color);
    }
}

pub fn draw_image(settings: &mut FrameBufferSettings, w: usize, h: usize, data: &[u8]) {
    let mut data_idx = 0;

    for y in 0..h {
        for x in 0..w {
            let r = data[data_idx] as u32;
            let g = data[data_idx + 1] as u32;
            let b = data[data_idx + 2] as u32;
            data_idx += 3;

            let pixel_color = (r << 16) | (g << 8) | b;

            draw_pixel(
                settings.address,
                settings.pitch,
                (settings.width / 2) + x as u32 - (w / 2) as u32,
                (settings.height / 2) + y as u32 - (h / 2) as u32,
                pixel_color,
            );
        }
    }
}
