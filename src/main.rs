#![feature(asm)]
#![feature(alloc)]
#![feature(heap_api)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]

extern crate alloc;
extern crate byteorder;
extern crate capstone;
extern crate docopt;
extern crate libc;
extern crate rustc_serialize;
extern crate time;

mod ast;
mod class;
mod cpu;
mod ctxt;
mod driver;
mod dseg;
mod error;
mod execstate;
mod gc;
mod interner;
mod jit;
mod lexer;
mod mem;
mod object;
mod os;
mod os_cpu;
mod parser;
mod semck;
mod stacktrace;
mod stdlib;
mod sym;
mod ty;
mod vtable;

#[cfg(test)]
mod test;

#[cfg(not(test))]
use std::process::exit;

#[cfg(not(test))]
fn main() {
    exit(driver::start());
}
