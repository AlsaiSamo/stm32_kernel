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
    //TODO: rewrite to use a linked list
    use core::ptr::swap;
    
    //Arbitrary number of threads. TODO: set at 16
    const MAX_THREAD_COUNT: usize = 4;

    #[repr(C)]
    #[derive(PartialEq, Eq, Copy, Clone)]
    pub enum Priority{
        Realtime,
        High,
        Medium,
        Low
    }

    //TODO: understand what states make sense
    #[repr(C)]
    #[derive(PartialEq, Eq, Copy, Clone)]
    pub enum State {
        Active,
        Sleeping,
        Dead
    }

    //TODO: make fields private
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Thread {
        pub id: usize,
        pub name: [char; 16],
        pub priority: Priority,
        pub state: State
    }
    
    impl Thread {
        const fn const_default() -> Self {
            Thread {
                id: 0,
                name: ['-'; 16],
                priority: Priority::Medium,
                state: State::Dead
            }
        }
    }
    
    pub struct ThreadQueue{
        queue: [Thread; MAX_THREAD_COUNT],
        len: usize
    }
    
    impl ThreadQueue {
        pub const fn default() -> Self {
            ThreadQueue {
                len: 0,
                queue: [Thread::const_default(); MAX_THREAD_COUNT]
            }
        }
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
        pub fn select(&mut self, prio: Priority) -> Option<&Thread> {
            let mut selection: Option<usize> = None;
            let mut i = 0;
            //Prevents from stopping seeking priority tasks
            let mut seeking_realtime = false;
            while i < self.len {
                if self.queue[i].state != State::Active{ i+= 1; continue; }
                let current = &self.queue[i];
                match (current, selection) {
                    (cur, _) if cur.priority == Priority::Realtime => {
                        selection = Some(i);
                        break;
                    },
                    (cur, _) if cur.priority == prio => {
                        if !seeking_realtime {
                            selection = Some(i);
                            seeking_realtime = true;
                        }
                        //break;
                    },
                    (_, None) => {selection = Some(i);},
                    (_, _) => {}
                }
                i += 1;
            }
            if let Some(i) = selection {
                unsafe{
                    self.queue[i..self.len].rotate_left(1);
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

use crate::threading::{Thread, Priority, State, ThreadQueue};

static mut threadQueue: ThreadQueue = ThreadQueue::default();
static mut USART1: USART = USART(0x40013800 as *mut usize);

#[panic_handler]
#[no_mangle]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    //TODO: have proper panicking behaviour
    loop{}
}

#[no_mangle]
pub unsafe extern "C" fn SysTick() -> () {
    unsafe {
        let sel = threadQueue.select(Priority::Medium).unwrap();
        USART1.test_send(&(sel.name[0] as u8));
    }
}


entrypoint!(main);
#[no_mangle]
pub fn main() -> ! {
    unsafe{
        threadQueue.add(Thread{
            id: 1,
            name: ['A'; 16],
            priority: Priority::Medium,
            state: State::Active
        }).unwrap();
        threadQueue.add(Thread{
            id: 3,
            name: ['B'; 16],
            priority: Priority::Medium,
            state: State::Active
        }).unwrap();
        threadQueue.add(Thread{
            id: 3,
            name: ['C'; 16],
            priority: Priority::Medium,
            state: State::Active
        }).unwrap();
    }
    //SysTick
    unsafe {
        //Configure systick
        ptr::write_volatile(0xE000E014 as *mut usize, 0x00010000);
        //Enable systick
        ptr::write_volatile(0xE000E010 as *mut usize, 0x00000003);
    }

    // let mut USART1 = USART(0x40013800 as *mut usize);
    // let _c = [60,61,62,63,64,65];
    // unsafe {
    //     USART1.test_send(&_c[5]);
    // }
    loop{}
}
