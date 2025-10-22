use core::{fmt::Display, ptr::null_mut};

use aos_uefi::{
    fs::{file::File, fileinfo::FileInfo, sfs::SimpleFileSystem},
    guid::Guid,
    status::Status,
};

pub struct FileSystem(&'static SimpleFileSystem);

impl From<&'static SimpleFileSystem> for FileSystem {
    fn from(value: &'static SimpleFileSystem) -> Self {
        Self(value)
    }
}

impl FileSystem {
    pub const GUID: Guid = Guid::new(
        0x0964e5b22,
        0x6459,
        0x11d2,
        [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
    );

    pub fn root(&self) -> Result<DirObject, Status> {
        let mut file_ptr: *mut File = null_mut();
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

pub struct FileInfoObject(FileInfo);

impl From<&'static FileInfo> for FileInfoObject {
    fn from(value: &'static FileInfo) -> Self {
        Self(*value)
    }
}

impl From<FileInfo> for FileInfoObject {
    fn from(value: FileInfo) -> Self {
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

pub struct FileObject(&'static File);

pub struct DirObject(FileObject);

impl From<&'static File> for DirObject {
    fn from(value: &'static File) -> Self {
        Self(FileObject(value))
    }
}

impl DirObject {
    pub fn next_entry(&self) -> Result<Option<FileInfoObject>, Status> {
        let mut info = FileInfo::default();
        let mut len = size_of::<FileInfo>();
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
