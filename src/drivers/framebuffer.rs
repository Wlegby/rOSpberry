#[repr(C, align(16))]
pub struct FramebufferMessage {
    pub message_size: u32,
    pub response_code: u32,
    // Tags go here...
    pub tag_set_physical_display: PropertyTag<2>, // e.g., 1920x1080
    pub tag_set_virtual_display: PropertyTag<2>,  // e.g., 1920x1080
    pub tag_set_depth: PropertyTag<1>,            // e.g., 32 bits per pixel (ARGB)
    pub tag_allocate_buffer: PropertyTag<1>,      // GPU fills this with the pointer
    // ...
    pub end_tag: u32, // Must be 0
}

#[repr(C)]
pub struct PropertyTag<const N: usize> {
    pub tag_id: u32, // Magic number for the specific command
    pub value_buffer_size: u32,
    pub request_response_code: u32,
    pub values: [u32; N], // Variable length depending on the tag
}
