#![no_std]

#[repr(C)]
pub struct BootInfo {
    frame_buffer: FrameBuffer,
}

#[repr(C)]
pub struct FrameBuffer {}
