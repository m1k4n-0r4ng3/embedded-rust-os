#![no_main]
#![no_std]

use core::panic::PanicInfo;
use core::ptr;
use cortex_m_semihosting::hprintln;
mod systick;

pub union Vector {
    reserved: u32,
    handler: unsafe extern "C" fn(),
}

extern "C" {
    fn NMI();
    fn HardFault();
    fn MemManage();
    fn BusFault();
    fn UsageFault();
    fn SVCall();
    fn PendSV();
    //fn SysTick();
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

#[no_mangle]
pub extern "C" fn SysTick() {
    hprintln!("Systick interrupt");
}

// The reset vector, a pointer into the reset handler
#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;

#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    extern "C" {
        static mut _sbss: u8;
        static mut _ebss: u8;
        static mut _sidata: u8;
        static mut _sdata: u8;
        static mut _edata: u8;
    }

    let count = &raw const _ebss as *const u8 as usize - &raw const _sbss as *const u8 as usize;
    ptr::write_bytes(&raw mut _sbss as *mut u8, 0, count);

    let count = &raw const _edata as *const u8 as usize - &raw const _sdata as *const u8 as usize;
    ptr::copy_nonoverlapping(&raw mut _sidata as *const u8, &raw mut _sdata as *mut u8, count);

    hprintln!("Reset");

    systick::init();
    
    loop {}
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}