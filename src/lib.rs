#![no_std]

use libc::write;
use core::ffi::c_void;

pub fn run() {
    let stdout = 1;
    let message = "Hello, World!\n";
    unsafe {
        write(stdout, message.as_ptr() as *const c_void, message.len());
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;

    #[test]
    fn test_run_ok() {
        run();
    }
}

