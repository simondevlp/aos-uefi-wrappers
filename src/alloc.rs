use core::{
    alloc::{GlobalAlloc, Layout},
    ptr::null_mut,
};

use aos_uefi::{memory::MemoryType, status::Status};

use crate::{
    println,
    system::{system_table, system_table_mut},
};

struct Allocator;

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut ret = 0;

        let st = unsafe { system_table_mut() };
        let status = (st.boot_srv.alloc_pool)(MemoryType::LoaderData, layout.size(), &mut ret);

        match status.0 {
            _s if !Status(_s).is_error() => ret as *mut u8,
            _s => null_mut(),
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        match unsafe { (system_table().boot_srv.free_pool)(ptr as usize) } {
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
