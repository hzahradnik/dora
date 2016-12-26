use std::ptr;

use baseline::fct::ExHandler;
use cpu::*;
use execstate::ExecState;
use mem;
use object::{Handle, Obj};
use stacktrace::StackFrameInfo;

pub use self::param::*;
pub use self::reg::*;

pub mod asm;
pub mod param;
pub mod reg;

pub fn sfi_from_execution_state(es: &ExecState) -> StackFrameInfo {
    let ra = unsafe { *(es.sp as *const usize) };

    StackFrameInfo {
        last: ptr::null(),
        fp: es.regs[RBP.int() as usize],
        sp: es.sp + mem::ptr_width() as usize,
        ra: ra,
        xpc: ra - 1,
    }
}

pub fn resume_with_handler(es: &mut ExecState,
                           handler: &ExHandler,
                           fp: usize,
                           exception: Handle<Obj>,
                           stacksize: usize) {
    if let Some(offset) = handler.offset {
        let arg = (fp as isize + offset as isize) as usize;

        unsafe {
            *(arg as *mut usize) = exception.raw() as usize;
        }
    }

    es.regs[RSP.int() as usize] = fp - stacksize;
    es.regs[RBP.int() as usize] = fp;
    es.pc = handler.catch;
}

pub fn flush_icache(_: *const u8, _: usize) {
    // no flushing needed on x86_64, but emit compiler barrier

    unsafe {
        asm!("" ::: "memory" : "volatile");
    }
}

pub fn get_exception_object(es: &ExecState) -> Handle<Obj> {
    let obj: Handle<Obj> = es.regs[REG_RESULT.int() as usize].into();

    obj
}

pub fn fp_from_execstate(es: &ExecState) -> usize {
    es.regs[RBP.int() as usize]
}

pub fn ra_from_execstate(es: &ExecState) -> usize {
    unsafe { *(es.sp as *const usize) }
}
