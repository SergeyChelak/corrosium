// use bootinfo::FrameBufferInfo;
// use uefi::{
//     boot::{self, ScopedProtocol},
//     proto::console::gop::{GraphicsOutput, Mode, PixelFormat},
// };

// const GRAPHICS_WIDTH: usize = 1920;
// const GRAPHICS_HEIGHT: usize = 1080;

// pub struct GOP {
//     width: usize,
//     height: usize,
// }

// impl Default for GOP {
//     fn default() -> Self {
//         Self::new(GRAPHICS_WIDTH, GRAPHICS_HEIGHT)
//     }
// }

// impl GOP {
//     pub fn new(width: usize, height: usize) -> Self {
//         Self { width, height }
//     }

//     pub fn framebuffer_info(&self) -> uefi::Result<FrameBufferInfo> {
//         let mut protocol = Self::get_gop()?;
//         let mode = self.choose_mode(&mut protocol)?;
//         protocol.set_mode(&mode)?;

//         let (width, height) = mode.info().resolution();
//         let stride = mode.info().stride();
//         let mut fb = protocol.frame_buffer();
//         let base_address = fb.as_mut_ptr() as *mut core::ffi::c_void;
//         let size = fb.size();
//         let pixel_format = match mode.info().pixel_format() {
//             PixelFormat::Rgb => bootinfo::PixelFormat::RGB,
//             PixelFormat::Bgr => bootinfo::PixelFormat::BGR,
//             _ => return Err(uefi::Status::UNSUPPORTED.into()),
//         };

//         let frame_buffer = FrameBufferInfo {
//             width,
//             height,
//             stride,
//             base_address,
//             size,
//             pixel_format,
//         };
//         Ok(frame_buffer)
//     }

//     fn get_gop() -> uefi::Result<ScopedProtocol<GraphicsOutput>> {
//         let handle = uefi::boot::get_handle_for_protocol::<GraphicsOutput>()?;
//         boot::open_protocol_exclusive::<GraphicsOutput>(handle)
//     }

//     fn choose_mode(&self, protocol: &mut ScopedProtocol<GraphicsOutput>) -> uefi::Result<Mode> {
//         protocol
//             .modes()
//             .into_iter()
//             .filter(|mode| {
//                 matches!(
//                     mode.info().pixel_format(),
//                     PixelFormat::Rgb | PixelFormat::Bgr
//                 )
//             })
//             .map(|mode| {
//                 let (width, height) = mode.info().resolution();
//                 let sqr_diff = width.abs_diff(self.width) + height.abs_diff(self.height);
//                 (mode, sqr_diff)
//             })
//             .min_by_key(|(_, sqr_diff)| *sqr_diff)
//             .map(|(mode, _)| mode)
//             .ok_or(uefi::Status::NOT_FOUND.into())
//     }
// }
