use core::arch::asm;
use core::marker::PhantomData;

#[repr(C)]
pub struct ContextFrame {
    pub r0: u32,
    pub r1: u32,
    pub r2: u32,
    pub r3: u32,
    pub r12: u32,
    pub lr: u32,
    pub return_addr: u32,
    pub xpsr: u32,
}

pub struct Process<'a> {
    sp: usize,
    regs: [u32; 8],
    marker: PhantomData<&'a u8>,
}

impl<'a> Process<'a> {
    pub fn new(stack: *mut u8, stack_len: &usize, app_main: extern "C" fn() -> !) -> Self {
        let sp = (stack as *const u8 as usize) + stack_len - 0x20;
        let context_frame: &mut ContextFrame = unsafe { &mut *(sp as *mut ContextFrame) };
        context_frame.r0 = 0;
        context_frame.r1 = 0;
        context_frame.r2 = 0;
        context_frame.r3 = 0;
        context_frame.r12 = 0;
        context_frame.lr = 0;
        context_frame.return_addr = app_main as u32;
        context_frame.xpsr = 0x0100_0000;

        Process {
            sp,
            regs: [0; 8],
            marker: PhantomData,
        }
    }

    pub fn exec(&mut self) {
        unsafe {
            asm!(
                "msr psp, {sp}",
                "ldmia r1, {{r4-r11}}",
                "svc 0",
                "stmia {regs}, {{r4-r11}}",
                "mrs {sp}, psp",
                sp = inout(reg) self.sp,
                regs = in(reg) self.regs.as_ptr(),
                out("r4") _,
                out("r5") _,
                out("r8") _,
                out("r9") _,
                out("r10") _,
                out("r11") _,
                options(nostack, preserves_flags),
            );
        }
    }
}