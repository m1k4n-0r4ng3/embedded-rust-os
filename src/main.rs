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
use process::Process;

mod linked_list;
use linked_list::ListItem;

mod scheduler;
use scheduler::Scheduler;

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

    #[link_section = ".app_stack"]
    static mut APP_STACK: [u8; 2048] = [0; 2048];
    static APP_STACK_LEN: usize = 2048;
    #[link_section = ".app_stack"]
    static mut APP_STACK2: [u8; 2048] = [0; 2048];
    static APP_STACK2_LEN: usize = 2048;
    #[link_section = ".app_stack"]
    static mut APP_STACK3: [u8; 2048] = [0; 2048];
    static APP_STACK3_LEN: usize = 2048;

    let mut process1 = Process::new(&raw mut APP_STACK as *mut u8, &APP_STACK_LEN, app_main);
    let mut item1 = ListItem::new(process1);
    let mut process2 = Process::new(&raw mut APP_STACK2 as *mut u8, &APP_STACK2_LEN, app_main2);
    let mut item2 = ListItem::new(process2);
    let mut process3 = Process::new(&raw mut APP_STACK3 as *mut u8, &APP_STACK3_LEN, app_main3);
    let mut item3 = ListItem::new(process3);

    let mut sched = Scheduler::new();
    sched.push(&mut item1);
    sched.push(&mut item2);
    sched.push(&mut item3);

    hprintln!("Kernel");
    hprintln!("App Start");

    sched.exec();

    
}

extern "C" fn app_main() -> ! {
    let mut i = 0;
    loop {
        hprintln!("APP1: {}", i);
        unsafe { 
            asm!("svc 0");
        }
        i += 1;
    }
}

extern "C" fn app_main2() -> ! {
    loop {
        hprintln!("APP2");
        unsafe { 
            asm!("svc 0");
        }
    }
}

extern "C" fn app_main3() -> ! {
    loop {
        hprintln!("APP3");
        unsafe { 
            asm!("svc 0");
        }
    }
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}