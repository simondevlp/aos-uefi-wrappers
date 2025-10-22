use core::{
    alloc::{GlobalAlloc, Layout},
    ptr::{null, null_mut},
    slice,
};

use aos_uefi::memory::Type;

use crate::{
    Status, println,
    system::{system_table, system_table_mut},
};

struct Allocator;

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut ret = 0 as *mut u8;

        let st = unsafe { system_table_mut() };
        let status = (st.boot_srv.alloc_pool)(Type::LoaderData, layout.size(), &mut ret);

        match status.0 {
            _s if !Status(_s).is_error() => ret as *mut u8,
            _s => null_mut(),
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        match unsafe { (system_table().boot_srv.free_pool)(ptr) } {
            Status::SUCCESS => (),
            _s => panic!("Allocator failed here at dealloc... Status {}", _s),
        }
    }
}

#[alloc_error_handler]
fn alloc_error(layout: Layout) -> ! {
    println!(
        "out of memory, size {}, aligned {}",
        layout.size(),
        layout.align()
    );
    loop {}
}

#[global_allocator]
static ALLOCATOR: Allocator = Allocator;

pub struct Allocated {
    ptr: *mut u8,
    layout: Layout,
}

impl Allocated {
    pub const ALIGN: usize = 8;

    pub fn null() -> Self {
        let layout = Layout::from_size_align(0, Self::ALIGN).unwrap();
        Self {
            ptr: null_mut(),
            layout: layout,
        }
    }

    pub fn new(size: usize) -> Self {
        let layout = Layout::from_size_align(size, Self::ALIGN).unwrap();
        Self {
            ptr: unsafe { ALLOCATOR.alloc(layout) },
            layout,
        }
    }

    pub fn slice_mut(&mut self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self.ptr, self.layout.size()) }
    }

    pub fn ptr_mut(&mut self) -> *mut u8 {
        self.ptr
    }

    pub fn set_at(&mut self, index: usize, val: u8) -> *const u8 {
        if index >= self.layout.size() {
            return null();
        }
        unsafe {
            *self.ptr.add(index) = val;
            self.ptr.add(index)
        }
    }

    pub fn set_slice_at(&mut self, index: usize, length: usize, val: &[u8]) -> *const u8 {
        if val.len() < length {
            return null();
        }
        unsafe {
            for i in 0..length {
                if self.set_at(index + i, val[i]).is_null() {
                    return null();
                }
            }
            self.ptr.add(index)
        }
    }

    pub fn get_at(&self, index: usize) -> *const u8 {
        if index >= self.layout.size() {
            return null();
        }
        unsafe { self.ptr.add(index) }
    }

    pub fn free(&self) {
        if self.ptr.is_null() {
            return;
        }
        unsafe {
            ALLOCATOR.dealloc(self.ptr, self.layout);
        }
    }
}

impl Drop for Allocated {
    fn drop(&mut self) {
        unsafe {
            ALLOCATOR.dealloc(self.ptr, self.layout);
        }
    }
}
