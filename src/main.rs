#![no_std]
#![no_main]

extern crate flight_brain;

#[no_mangle]
pub extern "C" fn main() {
    flight_brain::run();
}

#[cfg(not(test))]
use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

