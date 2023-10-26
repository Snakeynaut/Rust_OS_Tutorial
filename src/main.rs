#![no_std]
#![no_main] //we don't want the normal entry point chain

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
   loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
   loop {

   }
}