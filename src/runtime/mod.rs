use core::ptr;

pub mod usart{
    use core::marker::PhantomData;

    pub trait TransmitMode {}
    pub trait UsageType {}
    pub trait ConnectionStatus {}
    
    //Transmit modes
    pub struct Full;            //full duplex
    pub struct Half;            //half duplex
    pub struct Transmit;
    pub struct Receive;
    impl TransmitMode for Full{}
    impl TransmitMode for Half{}
    impl TransmitMode for Transmit{}
    impl TransmitMode for Receive{}

    //Usage type
    pub struct Direct;          //USART is used directly
    pub struct Managed;         //USART is used with DMA
    impl UsageType for Direct{}
    impl UsageType for Managed{}
    
    //Connection status
    pub struct Active;          //USART is being used
    pub struct Inactive;        //USART is not used by anything
    pub struct DataError;       //USART had received incorrect data
    impl ConnectionStatus for Active{}
    impl ConnectionStatus for Inactive{}
    impl ConnectionStatus for DataError{}

    pub enum USARTError{
        Overrun,
        Parity,
        Noise,
        Framing
    }
    
    //TODO: support full and half duplex
    //TODO: support DMA
    //TODO: configuration functions
    #[repr(C)]
    pub struct USART<T: TransmitMode, U: UsageType, S: ConnectionStatus> {
        address: *mut usize,
        _transmit_mode: PhantomData<T>,
        _usage_type: PhantomData<U>,
        _status: PhantomData<S>
    }
    //All states
    impl<T: TransmitMode, U: UsageType, S: ConnectionStatus> USART<T, U, S> {
        fn new(addr: *mut usize) -> USART<Receive, Direct, Inactive>{
            unimplemented!();
        }
        pub fn into_receive(&mut self) -> USART<Receive, U, S>{
            unimplemented!();
        }
        pub fn into_transmit(&mut self) -> USART<Transmit, U, S>{
            unimplemented!();
        }
    }
    //Active
    impl<T: TransmitMode, U: UsageType> USART<T, U, Active>{
        pub fn into_inactive(&mut self) -> USART<T, U, Inactive>{
            unimplemented!();
        }
    }
    //Inactive
    impl<T: TransmitMode, U: UsageType> USART<T, U, Inactive>{
        pub fn into_active(&mut self) -> USART<T, U, Active>{
            unimplemented!();
        }
    }
    //Active receiver
    impl<U: UsageType> USART<Receive, U, Active> {
        pub fn collect_data(&self) -> Result<(), USARTError>{
            unimplemented!();
        }
        pub fn into_errored(&mut self) -> USART<Receive, U, DataError>{
            unimplemented!();
        }
    }
    //Errored receiver
    impl<U: UsageType> USART<Receive, U, DataError>{
        pub fn clear_errors(&mut self) -> USART<Receive, U, Active>{
            unimplemented!();
        }
    }
    //Active transmitter
    impl<U: UsageType> USART<Transmit, U, Active>{
        pub fn send_data(&mut self){
            unimplemented!();
        }
    }
    //Transmitter cannot error
}

//Entrypoint type check
#[macro_export]
macro_rules! entrypoint {
    ($path:path) => (
        pub unsafe fn __main() -> ! {
            let f: fn() -> ! = $path;
            f()
        }
    )
}

#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {

    //Initialise memory
    extern "C" {
        static mut _sbss    :u8;
        static mut _ebss    :u8;
        static mut _sdata   :u8;
        static mut _edata   :u8;
        static     _sidata  :u8;
    }

    let count = &_ebss as *const u8 as usize - &_sbss as *const u8 as usize;
    ptr::write_bytes(&mut _sbss as *mut u8, 0, count);

    let count = &_edata as *const u8 as usize - &_sdata as *const u8 as usize;
    ptr::copy_nonoverlapping(&_sidata as *const u8, &mut _sdata as *mut u8, count);

    //Call the entrypoint
    extern "Rust" {
        fn main() -> !;
    }
    main()
}

#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;

pub union Vector {
    reserved: u32,
    handler: unsafe extern "C" fn()
}

//TODO: macro to namecheck/typecheck exceptions

extern "C" {
    fn NMI();
    fn HardFault();
    fn MemManage();
    fn BusFault();
    fn UsageFault();
    fn SVCall();
    fn PendSV();
    fn SysTick();
}

#[link_section = ".vector_table.exceptions"]
#[no_mangle]
pub static EXCEPTIONS: [Vector; 14] = [
    Vector { handler: NMI },
    Vector { handler: HardFault },
    Vector { handler: MemManage },
    Vector { handler: BusFault },
    Vector { handler: UsageFault },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { handler: SVCall },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { handler: PendSV },
    Vector { handler: SysTick },
];

#[no_mangle]
pub extern "C" fn DefaultExceptionHandler() {
    loop{}
}

//TODO: IRQs
