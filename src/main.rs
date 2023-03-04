#![no_std]
#![no_main]

use panic_abort as _;

extern "C" {
    fn the_answer() -> core::ffi::c_int;
}

// I know this doesn't work, but the point is it should build
#[no_mangle]
pub fn main() -> core::ffi::c_int {
    // SAFETY: Risking safety for The Answer? Worth it!
    unsafe { the_answer() }
}
