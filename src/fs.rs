use core::{fmt::Display, ptr::null_mut};

use crate::Status;

pub struct FileSystem(&'static aos_uefi::fs::sfs::Protocol);

impl From<&'static aos_uefi::fs::sfs::Protocol> for FileSystem {
    fn from(value: &'static aos_uefi::fs::sfs::Protocol) -> Self {
        Self(value)
    }
}

impl FileSystem {
    pub fn root(&self) -> Result<DirObject, Status> {
        let mut file_ptr: *mut aos_uefi::fs::file::Protocol = null_mut();
        let status = (self.0.open_volume)(self.0, &mut file_ptr);

        match status {
            Status::SUCCESS => {
                if file_ptr.is_null() {
                    Err(Status::INVALID_PARAMETER)
                } else {
                    unsafe { Ok(DirObject::from(&*file_ptr)) }
                }
            }
            _s => Err(_s),
        }
    }
}

pub struct FileInfoObject(aos_uefi::fs::fileinfo::FileInfo);

impl FileInfoObject {
    pub fn name(&self) -> [u16; 256] {
        self.0.file_name
    }

    pub fn cmp_name(&self, rhs: &[u16]) -> bool {
        for i in 0..256 {
            if rhs[i] != self.0.file_name[i] {
                return false;
            }
            if rhs[i] == 0 {
                return true;
            }
        }
        false
    }
}

impl From<&'static aos_uefi::fs::fileinfo::FileInfo> for FileInfoObject {
    fn from(value: &'static aos_uefi::fs::fileinfo::FileInfo) -> Self {
        Self(*value)
    }
}

impl From<aos_uefi::fs::fileinfo::FileInfo> for FileInfoObject {
    fn from(value: aos_uefi::fs::fileinfo::FileInfo) -> Self {
        Self(value)
    }
}

impl Display for FileInfoObject {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for b in self.0.file_name {
            if b == 0 {
                return Ok(());
            }
            if b == '\n' as u16 {
                write!(f, "\r\n")?;
            } else if b <= 0x7F {
                // Convert UTF-16 to ASCII for basic characters
                write!(f, "{}", b as u8 as char)?;
            } else {
                // For non-ASCII characters, show hex
                write!(f, "\\u{:04x}", b)?;
            }
        }
        Ok(())
    }
}

pub struct FileObject(&'static aos_uefi::fs::file::Protocol);

pub struct DirObject(FileObject);

impl From<&'static aos_uefi::fs::file::Protocol> for DirObject {
    fn from(value: &'static aos_uefi::fs::file::Protocol) -> Self {
        Self(FileObject(value))
    }
}

impl DirObject {
    pub fn next_entry(&self) -> Result<Option<FileInfoObject>, Status> {
        let mut info = aos_uefi::fs::fileinfo::FileInfo::default();
        let mut len = size_of::<aos_uefi::fs::fileinfo::FileInfo>();
        let status = (self.0.0.read)(self.0.0, &mut len, &mut info as *mut _ as *mut u8);

        match status {
            Status::SUCCESS => match len {
                0 => Ok(None),
                _ => Ok(Some(FileInfoObject::from(info))),
            },
            _s => Err(_s),
        }
    }
}
