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

pub mod threading {
    use core::ptr::swap;
    
    const MAX_THREAD_COUNT: usize = 16;

    #[repr(C)]
    #[derive(PartialEq, Eq)]
    pub enum Priority{
        Realtime,
        High,
        Medium,
        Low
    }

    //TODO: understand what states make sense
    #[repr(C)]
    #[derive(PartialEq, Eq)]
    pub enum State {
        Active,
        Sleeping,
        Dead
    }

    //TODO: make a default thread
    #[repr(C)]
    pub struct Thread {
        id: usize,
        name: [char; 16],
        priority: Priority,
        state: State
    }
    
    //Arbitrary number of threads - 16
    //struct ThreadQueue([Thread; 16]);
    struct ThreadQueue{
        queue: [Thread; MAX_THREAD_COUNT],
        len: usize
    }

    impl ThreadQueue {
        ///Selects the thread that should be run next. The selected thread is pushed to the end of the queue and its reference is returned.
        ///
        /// The algorhitm:
        ///1. The first encountered Active Realtime thread will be selected, if such exists.
        ///2. The first encountered Active thread with priority matching the given requirement will
        ///   be selected, if such exists.
        ///3. The first Active thread will be selected, if such exists.
        ///4. No thread is selected
        fn select(&mut self, prio: Priority) -> Option<&Thread> {
            let mut selection: Option<usize> = None;
            let mut i = 0;
            while i < self.len {
                if self.queue[i].state != State::Active{ i+= 1; continue; }
                let current = &self.queue[i];
                match (current, selection) {
                    (cur, _) if cur.priority == Priority::Realtime => {
                        selection = Some(i);
                        break;
                    },
                    (cur, _) if cur.priority == prio => {
                        selection = Some(i);
                        break;
                    },
                    (_, None) => {selection = Some(i);},
                    (_, _) => {}
                }
            }
            if let Some(i) = selection {
                unsafe{
                    swap(&mut self.queue[i] as *mut Thread, &mut self.queue[self.len-1] as *mut Thread);
                }
                return Some(&self.queue[self.len-1]);
            }
            None
        }
        pub fn add(&mut self, thread: Thread) -> Result<(),()> {
            if self.len >= MAX_THREAD_COUNT { return Err(()) }
            self.queue[self.len] = thread;
            self.len += 1;
            Ok(())
        }
    }
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    //TODO: have proper panicking behaviour
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
