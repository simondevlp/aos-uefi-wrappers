use core::fmt::{Arguments, Write};

use aos_uefi::status::Status;

use crate::system::{self, system_table};

pub struct Stdout;

pub fn _print(args: Arguments) {
    Stdout.write_fmt(args).unwrap();
}

pub fn _println(args: Arguments) {
    Stdout.write_fmt(args).unwrap();
    Stdout.write_char('\n').unwrap();
}

impl Stdout {
    pub fn clear_screen(&self) -> Result<(), Status> {
        unsafe {
            let cout = system::system_table().cout;
            match (cout.clear)(cout) {
                Status::SUCCESS => Ok(()),
                _s => Err(_s),
            }
        }
    }
}

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        unsafe {
            let cout = system_table().cout;
            for &ch in s.as_bytes() {
                if ch == b'\n' {
                    (cout.output)(&cout, [b'\r' as u16, b'\n' as u16, 0].as_ptr());
                } else {
                    (cout.output)(&cout, [ch as u16, 0].as_ptr());
                }
            }
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($e:tt)*) => {
        $crate::wrappers::stdio::_print(format_args!($($e)*))
    };
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n")
    };
    ($($e:tt)*) => {
        $crate::stdio::_println(format_args!($($e)*))
    };
}
