#![no_main]
#![no_std]
#![feature(naked_functions)]

use core::panic::PanicInfo;
use core::ptr;
use core::arch::asm;
use core::arch::naked_asm;
use cortex_m_semihosting::hprintln;

mod systick;

mod process;
use process::ContextFrame;

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
    fn PendSV();
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

#[naked]
#[no_mangle]
pub unsafe extern "C" fn SVCall() {
    naked_asm!(
        "cmp lr, #0xfffffff9",
        "bne 1f",
        "mov r0, #1",
        "msr CONTROL, r0",
        "movw lr, #0xfffd",
        "movt lr, #0xffff",
        "bx lr",
        "1:",
        "mov r0, #0",
        "msr CONTROL, r0",
        "movw lr, #0xfff9",
        "movt lr, #0xffff",
        "bx lr",
    );
}

#[link_section = ".app_stack"]
static mut APP_STACK: [u8; 1024] = [0; 1024];

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

    let ptr = (&APP_STACK[0] as *const u8 as usize) + 1024 - 0x20;
    let context_frame: &mut ContextFrame = &mut *(ptr as *mut ContextFrame);

    context_frame.r0 = 0;
    context_frame.r1 = 0;
    context_frame.r2 = 0;
    context_frame.r3 = 0;
    context_frame.r12= 0;
    context_frame.lr = 0;
    context_frame.return_addr = app_main as u32;
    context_frame.xpsr = 0x0100_0000;

    asm!(
        "msr psp, r0",
        "svc 0",
        in("r0") ptr,
        out("r4") _,
        out("r5") _,
        out("r8") _,
        out("r9") _,
        out("r10") _,
        out("r11") _,
    );
    
    hprintln!("Kernel");
    loop {}
}

extern "C" fn app_main() -> ! {
    hprintln!("APP");
    unsafe { 
        asm!("svc 0");
    }
    loop{}
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}