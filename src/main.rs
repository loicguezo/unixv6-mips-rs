#![no_std]
#![feature(asm_experimental_arch)]
#![no_main]

use core::panic::PanicInfo;

mod arch;

#[unsafe(no_mangle)]
pub extern "C" fn kmain() -> ! {
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {}
}
