#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::ptr;
pub mod runtime;

//USART 1 at 0x40013800
#[repr(C)]
pub struct USART(*mut usize);
impl USART {
    pub unsafe fn test_send(&mut self, data: &u8) {
        //Test function - sends one character
        //BUG: issue is that QEMU does not emulate many of USART's workings.
        //I simply cannot write AND test code that will run on real hardware.
        unsafe {
            let DR: usize = {
                //This is 0x0 in QEMU and random in the docs
                let prev_dr = ptr::read_volatile(self.0.offset(1));
                let mask = 0x000001ff;
                let masked_data = (*data) as usize & mask;
                let cleared_dr = prev_dr & !mask;
                masked_data | cleared_dr
            };
            //Checking QEMU's USART implementation - should enable USAART and give an interrupt on
            //transaction end.
            ptr::write_volatile(self.0.offset(3), 0x00002020);
            ptr::write_volatile(self.0.offset(1), DR);
        }
    }
}


#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    //TODO: make cpu sleep?
    loop{}
}

#[no_mangle]
pub fn SysTick() -> () {
    loop{}
}

entrypoint!(main);

#[no_mangle]
pub fn main() -> ! {
    let mut USART1 = USART(0x40013800 as *mut usize);
    let _c = [60,61,62,63,64,65];
    unsafe {
        USART1.test_send(&_c[5]);
    }
    loop{}
}
