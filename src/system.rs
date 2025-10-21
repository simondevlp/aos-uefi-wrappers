use crate::fs;
use aos_uefi::{self as uefi, Handle, status::Status};

pub struct System;

impl System {
    pub fn bootsrv() -> &'static uefi::boot::BootServices {
        unsafe { system_table().boot_srv }
    }

    pub fn image_handle() -> &'static Handle {
        unsafe { image_handle() }
    }

    pub fn get_protocol(id: &uefi::guid::Guid) -> Result<usize, Status> {
        let mut ptr = 0usize;
        let status = (Self::bootsrv().locate_protocol)(id, 0, &mut ptr);
        match status {
            Status::SUCCESS => Ok(ptr),
            _s => {
                println!("Status: {}", _s);
                panic!()
            }
        }
    }

    pub fn get_fs() -> Result<fs::FileSystem, Status> {
        match Self::get_protocol(&fs::FileSystem::GUID) {
            Ok(u) => unsafe {
                Ok(fs::FileSystem::from(
                    &*(u as *mut uefi::fs::sfs::SimpleFileSystem),
                ))
            },
            Err(s) => Err(s),
        }
    }
}

static mut SYSTEM_TABLE: *mut uefi::system::SystemTable = 0 as *mut uefi::system::SystemTable;
static mut IMAGE_HANDLE: *mut uefi::Handle = 0 as *mut uefi::Handle;

pub unsafe fn system_table() -> &'static uefi::system::SystemTable {
    unsafe { &*SYSTEM_TABLE }
}

pub unsafe fn system_table_mut() -> &'static mut uefi::system::SystemTable {
    unsafe { &mut *SYSTEM_TABLE }
}

pub unsafe fn image_handle() -> &'static Handle {
    unsafe { &*IMAGE_HANDLE }
}

unsafe extern "C" {
    fn amain() -> Status;
}

#[unsafe(no_mangle)]
extern "efiapi" fn efi_main(
    image_handle: Handle,
    system_table: &'static mut uefi::system::SystemTable,
) -> Status {
    unsafe {
        SYSTEM_TABLE = &mut *system_table;
        *IMAGE_HANDLE = image_handle;
        match amain() {
            Status::SUCCESS => loop {},
            _s => {
                panic!("main failed returning status {}", _s)
            }
        }
    }
}
