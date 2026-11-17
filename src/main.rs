#![feature(asm_experimental_arch)]
#![no_std]
#![no_main]
#![allow(dead_code)]

use core::panic::PanicInfo;
mod arch;
mod param;
mod proc;

#[unsafe(no_mangle)]
pub extern "C" fn kmain() -> ! {
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {}
}
