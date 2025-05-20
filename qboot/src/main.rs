#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// Point d’entrée du bootloader
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let uart_base = 0x0900_0000 as *mut u8;
    let hello = b"QPhone OS Bootloader...\n";

    for &c in hello {
        unsafe {
            core::ptr::write_volatile(uart_base, c);
        }
    }

    loop {}
}

/// Handler d’erreur minimal
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}