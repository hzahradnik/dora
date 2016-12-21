use std::ptr;

use baseline::fct::CatchType;
use cpu::{resume_with_handler, fp_from_execstate};
use ctxt::{Context, FctKind, FctId, get_ctxt};
use object::{Handle, Obj};
use execstate::ExecState;

pub struct Stacktrace {
    elems: Vec<StackElem>
}

impl Stacktrace {
    pub fn new() -> Stacktrace {
        Stacktrace {
            elems: Vec::new()
        }
    }

    pub fn len(&self) -> usize {
        self.elems.len()
    }

    pub fn push_entry(&mut self, fct_id: FctId, lineno: i32) {
        self.elems.push(StackElem {
            fct_id: fct_id,
            lineno: lineno
        });
    }

    pub fn dump(&self, ctxt: &Context) {
        for (ind, elem) in self.elems.iter().rev().enumerate() {
            let name = ctxt.fct_by_id(elem.fct_id).full_name(ctxt);
            print!("  {}: {}:", ind, name);

            if elem.lineno == 0 {
                println!("?");
            } else {
                println!("{}", elem.lineno);
            }
        }
    }
}

struct StackElem {
    fct_id: FctId,
    lineno: i32
}

pub struct StackFrameInfo {
    pub last: *const StackFrameInfo,
    pub sp: usize,
    pub fp: usize,
    pub ra: usize,
    pub xpc: usize,
}

impl StackFrameInfo {
    pub fn new() -> StackFrameInfo {
        StackFrameInfo {
            last: ptr::null(),
            sp: 0,
            fp: 0,
            ra: 0,
            xpc: 0,
        }
    }
}

pub fn get_stacktrace(ctxt: &Context, es: &ExecState) -> Stacktrace {
    let mut stacktrace = Stacktrace::new();
    determine_stack_entry(&mut stacktrace, ctxt, es.pc);

    let mut fp = fp_from_execstate(es);

    while fp != 0 {
        let ra = unsafe { *((fp + 8) as *const usize) };
        let cont = determine_stack_entry(&mut stacktrace, ctxt, ra);

        if !cont { break; }

        fp = unsafe { *(fp as *const usize) };
    }

    return stacktrace;
}

fn determine_stack_entry(stacktrace: &mut Stacktrace, ctxt: &Context, pc: usize) -> bool {
    let code_map = ctxt.code_map.lock().unwrap();
    let fct_id = code_map.get(pc as *const u8);

    if let Some(fct_id) = fct_id {
        let mut lineno = 0;
        let fct = ctxt.fct_by_id(fct_id);
        if let FctKind::Source(ref src) = fct.kind {
            let src = src.lock().unwrap();
            let jit_fct = src.jit_fct.as_ref().unwrap();
            let offset = pc - (jit_fct.fct_ptr() as usize);
            lineno = jit_fct.lineno_for_offset(offset as i32);

            if lineno == 0 {
                panic!("lineno not found for program point");
            }
        }

        stacktrace.push_entry(fct_id, lineno);

        true
    } else {
        // only continue if we still haven't reached jitted functions
        stacktrace.len() == 0
    }
}

pub fn handle_exception(exception: Handle<Obj>, es: &mut ExecState) -> bool {
    let mut pc : usize = es.pc;
    let mut fp : usize = fp_from_execstate(es);

    loop {
        let found = find_handler(exception, es, pc, fp);

        match found {
            HandlerFound::Yes => { return true; }
            HandlerFound::Stop => { return false; }
            HandlerFound::No => {
                if fp == 0 { return false; }
            }
        }

        pc = unsafe { *((fp + 8) as *const usize) };
        fp = unsafe { *(fp as *const usize) };
    }
}

#[derive(PartialEq, Eq, Debug)]
enum HandlerFound { Yes, No, Stop }

fn find_handler(exception: Handle<Obj>, es: &mut ExecState, pc: usize, fp: usize) -> HandlerFound {
    let ctxt = get_ctxt();
    let fct_id = {
        let code_map = ctxt.code_map.lock().unwrap();
        code_map.get(pc as *const u8)
    };

    // println!("------------");
    // println!("find {:x}", pc);

    if let Some(fct_id) = fct_id {
        let fct = ctxt.fct_by_id(fct_id);

        if let FctKind::Source(ref src) = fct.kind {
            let src = src.lock().unwrap();

            if let Some(ref jit_fct) = src.jit_fct {
                let cls_id = exception.header().vtbl().class().id;

                for entry in &jit_fct.exception_handlers {
                    // println!("entry = {:x} to {:x} for {:?}",
                    //          entry.try_start, entry.try_end, entry.catch_type);

                    if entry.try_start < pc && pc <= entry.try_end
                        && (entry.catch_type == CatchType::Any
                            || entry.catch_type == CatchType::Class(cls_id)) {
                        let stacksize = src.stacksize() as usize;
                        resume_with_handler(es, entry, fp, exception, stacksize);

                        return HandlerFound::Yes;

                    } else if pc > entry.try_end {
                        // exception handlers are sorted, no more possible handlers
                        // in this function

                        return HandlerFound::No;
                    }
                }
            }

            // exception can only bubble up in stacktrace if current function
            // is allowed to throw exceptions
            if !fct.ast.throws {
                return HandlerFound::Stop;
            }
        }
    }

    HandlerFound::No
}
