use elf::{ElfBytes, endian::AnyEndian, segment::ProgramHeader};
use log::{error, info};
use uefi::{
    boot::{self, MemoryType},
    cstr16,
    proto::{
        loaded_image::LoadedImage,
        media::{
            file::{File, FileAttribute, FileInfo, FileMode, RegularFile},
            fs::SimpleFileSystem,
        },
    },
};

use crate::utils::wait_for_key;

pub struct KernelLoadInfo {
    pub start_addr: u64,
    pub end_addr: u64,
    pub entry_point: u64,
}

pub fn load_kernel() -> uefi::Result<KernelLoadInfo> {
    let mut file = open_file()?;

    // Get the file size
    // 128 bytes is generally more than enough for a standard FileInfo struct
    let mut info_buf = [0u8; 128];
    let info = file
        .get_info::<FileInfo>(&mut info_buf)
        .map_err(|_| uefi::Status::BUFFER_TOO_SMALL)?;
    let file_size = info.file_size() as usize;
    info!("Kernel size: {file_size}");

    // 3. Load the ELF file into memory
    let elf_data = boot::allocate_pool(MemoryType::LOADER_DATA, file_size)?;
    let buffer =
        unsafe { core::slice::from_raw_parts_mut(elf_data.as_ptr() as *mut u8, file_size) };
    file.read(buffer)?;

    let elf_file = match ElfBytes::<AnyEndian>::minimal_parse(&buffer) {
        Ok(f) => f,
        Err(_) => return Err(uefi::Status::LOAD_ERROR.into()),
    };

    // 4. Check that the ELF file is a 64-bit executable
    if elf_file.ehdr.class != elf::file::Class::ELF64 {
        error!("Kernel must be an ELF64 executable");
        return Err(uefi::Status::UNSUPPORTED.into());
    }

    let mut start_addr = u64::MAX;
    let mut end_addr = u64::MIN;

    let segments = elf_file.segments().ok_or(uefi::Status::LOAD_ERROR)?;

    for p in segments {
        if p.p_type == elf::abi::PT_LOAD {
            load_segment(&p, &buffer, &mut start_addr, &mut end_addr)?;
        }
    }

    let entry_point = elf_file.ehdr.e_entry;

    unsafe {
        boot::free_pool(elf_data)?;
    }

    Ok(KernelLoadInfo {
        start_addr,
        end_addr,
        entry_point,
    })
}

fn open_file() -> uefi::Result<RegularFile> {
    let loaded_image = boot::open_protocol_exclusive::<LoadedImage>(boot::image_handle())?;

    let Some(device_handle) = loaded_image.device() else {
        return Err(uefi::Status::DEVICE_ERROR.into());
    };

    let mut sfs = boot::open_protocol_exclusive::<SimpleFileSystem>(device_handle)?;
    let mut dir = sfs.open_volume()?;

    let filename = cstr16!("\\kernel.elf");
    let file_handle = dir.open(filename, FileMode::Read, FileAttribute::empty())?;

    file_handle
        .into_regular_file()
        .ok_or(uefi::Status::NOT_FOUND.into())
}

fn load_segment(
    p: &ProgramHeader,
    buffer: &[u8],
    start_addr: &mut u64,
    end_addr: &mut u64,
) -> uefi::Result<()> {
    let page_start = p.p_vaddr & !0xFFF;
    let page_end = (p.p_vaddr + p.p_memsz + 0xFFF) & !0xFFF;
    let num_pages = (page_end - page_start) / 0x1000;

    if num_pages == 0 {
        return Ok(());
    }

    let result = boot::allocate_pages(
        boot::AllocateType::Address(page_start),
        MemoryType::LOADER_DATA,
        num_pages as usize,
    );

    if let Err(e) = result {
        log::warn!(
            "Failed to allocate {} pages at {:#X} ({:?})",
            num_pages,
            page_start,
            e.status()
        );
        wait_for_key();
    }

    // Copy data
    let src = &buffer[p.p_offset as usize..(p.p_offset + p.p_filesz) as usize];
    let dst = unsafe { core::slice::from_raw_parts_mut(p.p_vaddr as *mut u8, p.p_filesz as usize) };
    dst.copy_from_slice(src);

    // Zero remaining memsz
    let bss_size = p.p_memsz - p.p_filesz;
    if bss_size > 0 {
        let bss = unsafe {
            core::slice::from_raw_parts_mut((p.p_vaddr + p.p_filesz) as *mut u8, bss_size as usize)
        };
        bss.fill(0);
    }

    *start_addr = p.p_vaddr.min(*start_addr);
    *end_addr = (p.p_vaddr + p.p_memsz).max(*end_addr);

    Ok(())
}
