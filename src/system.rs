use crate::{Guid, Handle, Status, fs};

pub struct System;

impl System {
    pub fn bootsrv() -> &'static aos_uefi::boot::BootServices {
        unsafe { system_table().boot_srv }
    }

    pub fn image_handle() -> &'static Handle {
        unsafe { image_handle() }
    }

    pub fn get_protocol(id: &Guid) -> Result<usize, Status> {
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
        match Self::get_protocol(&aos_uefi::fs::sfs::Protocol::GUID) {
            Ok(u) => unsafe {
                Ok(fs::FileSystem::from(
                    &*(u as *mut aos_uefi::fs::sfs::Protocol),
                ))
            },
            Err(s) => Err(s),
        }
    }

    pub fn get_devpathutils() -> Result<&'static aos_uefi::devpath::Utils, Status> {
        match Self::get_protocol(&aos_uefi::devpath::Utils::GUID) {
            Ok(u) => unsafe { Ok(&*(u as *const aos_uefi::devpath::Utils)) },
            Err(s) => Err(s),
        }
    }
}

static mut SYSTEM_TABLE: *mut aos_uefi::system::SystemTable =
    0 as *mut aos_uefi::system::SystemTable;
static mut IMAGE_HANDLE: *mut aos_uefi::Handle = 0 as *mut aos_uefi::Handle;

pub unsafe fn system_table() -> &'static aos_uefi::system::SystemTable {
    unsafe { &*SYSTEM_TABLE }
}

pub unsafe fn system_table_mut() -> &'static mut aos_uefi::system::SystemTable {
    unsafe { &mut *SYSTEM_TABLE }
}

pub unsafe fn image_handle() -> &'static Handle {
    unsafe { &*IMAGE_HANDLE }
}

unsafe extern "C" {
    fn main() -> Status;
}

#[unsafe(no_mangle)]
extern "efiapi" fn efi_main(
    image_handle: Handle,
    system_table: &'static mut aos_uefi::system::SystemTable,
) -> Status {
    unsafe {
        SYSTEM_TABLE = &mut *system_table;
        *IMAGE_HANDLE = image_handle;
        match main() {
            Status::SUCCESS => loop {},
            _s => {
                panic!("main failed returning status {}", _s)
            }
        }
    }
}
