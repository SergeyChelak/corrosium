use crate::memory::{KernelSpaceMemoryManager, PAGE_PRESENT, PAGE_WRITABLE};
use bootinfo::address::VirtualAddress;
use elf::{ElfBytes, endian::AnyEndian, segment::ProgramHeader};
use log::{error, info};
use uefi::{
    CStr16,
    boot::{self, MemoryType},
    proto::{
        loaded_image::LoadedImage,
        media::{
            file::{File, FileAttribute, FileInfo, FileMode, RegularFile},
            fs::SimpleFileSystem,
        },
    },
};

pub struct KernelLoadInfo {
    pub start_addr: VirtualAddress,
    pub end_addr: VirtualAddress,
    pub entry_point: VirtualAddress,
}

pub struct KernelLoader<'a> {
    filename: &'a CStr16,
    memory_manager: &'a mut KernelSpaceMemoryManager,
}

impl<'a> KernelLoader<'a> {
    pub fn new(filename: &'a CStr16, memory_manager: &'a mut KernelSpaceMemoryManager) -> Self {
        Self {
            filename,
            memory_manager,
        }
    }

    pub fn load(&mut self) -> uefi::Result<KernelLoadInfo> {
        let mut file = open_file(&self.filename)?;

        let file_size = get_file_size(&mut file)?;
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

        let mut start_addr = VirtualAddress::new(u64::MAX);
        let mut end_addr = VirtualAddress::new(u64::MIN);

        let segments = elf_file.segments().ok_or(uefi::Status::LOAD_ERROR)?;

        let mut load_result = Ok(());
        for p in segments.iter().filter(|ph| ph.p_type == elf::abi::PT_LOAD) {
            if let Err(e) = self.load_segment(&p, &buffer, &mut start_addr, &mut end_addr) {
                load_result = Err(e);
                break;
            }
        }

        let entry_point = VirtualAddress::new(elf_file.ehdr.e_entry);

        unsafe {
            if let Err(e) = boot::free_pool(elf_data) {
                error!("Failed to free ELF pool: {:?}", e);
            }
        }

        load_result?;

        Ok(KernelLoadInfo {
            start_addr,
            end_addr,
            entry_point,
        })
    }

    fn load_segment(
        &mut self,
        p: &ProgramHeader,
        buffer: &[u8],
        start_addr: &mut VirtualAddress,
        end_addr: &mut VirtualAddress,
    ) -> uefi::Result<()> {
        let page_start = p.p_vaddr & !0xFFF;
        let page_end = (p.p_vaddr + p.p_memsz + 0xFFF) & !0xFFF;
        let num_pages = ((page_end - page_start) / 0x1000) as usize;

        if num_pages == 0 {
            return Ok(());
        }

        // Determine flags based on segment flags.
        // We'll use WRITABLE unconditionally for now, but could be refined.
        let flags = PAGE_PRESENT | PAGE_WRITABLE;

        let phys_start = self
            .memory_manager
            .allocate_mapped(VirtualAddress::new(page_start), num_pages, flags)
            .map_err(|e| {
                error!(
                    "Failed to allocate {} pages at {:#X} ({:?})",
                    num_pages,
                    page_start,
                    e.status()
                );
                e.status()
            })?;

        let phys_segment_start = phys_start + (p.p_vaddr - page_start);

        // Copy data
        let src = &buffer[p.p_offset as usize..(p.p_offset + p.p_filesz) as usize];
        let dst = unsafe {
            core::slice::from_raw_parts_mut(
                phys_segment_start.as_u64() as *mut u8,
                p.p_filesz as usize,
            )
        };
        dst.copy_from_slice(src);

        // Zero remaining memsz
        let bss_size = p.p_memsz - p.p_filesz;
        if bss_size > 0 {
            let bss = unsafe {
                core::slice::from_raw_parts_mut(
                    (phys_segment_start + p.p_filesz).as_u64() as *mut u8,
                    bss_size as usize,
                )
            };
            bss.fill(0);
        }

        *start_addr = VirtualAddress::new(p.p_vaddr.min(start_addr.as_u64()));
        *end_addr = VirtualAddress::new((p.p_vaddr + p.p_memsz).max(end_addr.as_u64()));

        Ok(())
    }
}

fn open_file(filename: &CStr16) -> uefi::Result<RegularFile> {
    let loaded_image = boot::open_protocol_exclusive::<LoadedImage>(boot::image_handle())?;

    let Some(device_handle) = loaded_image.device() else {
        return Err(uefi::Status::DEVICE_ERROR.into());
    };

    let mut sfs = boot::open_protocol_exclusive::<SimpleFileSystem>(device_handle)?;
    let mut dir = sfs.open_volume()?;

    let file_handle = dir.open(filename, FileMode::Read, FileAttribute::empty())?;

    file_handle
        .into_regular_file()
        .ok_or(uefi::Status::NOT_FOUND.into())
}

fn get_file_size(file: &mut RegularFile) -> uefi::Result<usize> {
    // 128 bytes is generally more than enough for a standard FileInfo struct
    let mut info_buf = [0u8; 128];
    let info = file
        .get_info::<FileInfo>(&mut info_buf)
        .map_err(|_| uefi::Status::BUFFER_TOO_SMALL)?;
    let file_size = info.file_size() as usize;
    Ok(file_size)
}
