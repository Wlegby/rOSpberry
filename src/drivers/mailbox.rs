use crate::bsp::memory::VIDEOCORE_MAILBOX;
use core::ptr::{read_volatile, write_volatile};

const MBOX_READ: usize = VIDEOCORE_MAILBOX; // We get message from GPU here
const MBOX_STATUS: usize = VIDEOCORE_MAILBOX + 0x18; // contains information about the MBOX
const MBOX_WRITE: usize = VIDEOCORE_MAILBOX + 0x20; // we write here to give data to GPU

const MBOX_FULL: u32 = 0x8000_0000;
const MBOX_EMPTY: u32 = 0x4000_0000;

pub const MBOX_CHANNEL: u8 = 8;

// tries to write to mailbox (the data as a pointer)
pub fn mailbox_write(channel: u8, data: u32) {
    unsafe {
        // waits until mailbox isn't empty
        while read_volatile(MBOX_STATUS as *const u32) & MBOX_FULL != 0 {
            core::hint::spin_loop();
        }
        // creates message (adds the data and the channel at the lower bits of the data)
        let message = (data & !0xF) | (channel as u32 & 0xF);

        // writes message
        write_volatile(MBOX_WRITE as *mut u32, message);
    }
}

// waits for a reply from the gpu
pub fn mailbox_read(channel: u8) -> u32 {
    unsafe {
        loop {
            // waits for the mailbox not to be empty
            while read_volatile(MBOX_STATUS as *const u32) & MBOX_EMPTY != 0 {
                core::hint::spin_loop();
            }

            let data = read_volatile(MBOX_READ as *const u32);

            let read_channel = (data & 0xF) as u8;

            //makes sure it's the correct channel
            if read_channel == channel {
                // removes the channel from the data
                return data & !0xF;
            }
        }
    }
}

// Force 16-byte alignment
#[repr(C, align(16))]
pub struct MboxBuffer {
    // the payload
    pub buffer: [u32; 36],
}

impl MboxBuffer {
    pub const fn new() -> Self {
        Self { buffer: [0; 36] }
    }

    // gives the gpu the pointer to the data with the channel
    pub fn call(&self, channel: u8) -> bool {
        let buf_ptr = self.buffer.as_ptr() as u32;

        // writes the pointer to the mailbox
        mailbox_write(channel, buf_ptr);
        // waits for the response from the gpu
        mailbox_read(channel);

        // checks if the gpu wrote successfull into the second slot of the buffer
        self.buffer[1] == 0x8000_0000
    }
}
