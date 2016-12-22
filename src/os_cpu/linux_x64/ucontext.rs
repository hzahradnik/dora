#[repr(C)]
pub struct ucontext_t {
    _ignore: [u64; 5],
    pub uc_mcontext: mcontext_t,
}

#[repr(C)]
pub struct mcontext_t {
    pub regs: [usize; 23],
}

pub const REG_R8: usize = 0;
pub const REG_R9: usize = 1;
pub const REG_R10: usize = 2;
pub const REG_R11: usize = 3;
pub const REG_R12: usize = 4;
pub const REG_R13: usize = 5;
pub const REG_R14: usize = 6;
pub const REG_R15: usize = 7;
pub const REG_RDI: usize = 8;
pub const REG_RSI: usize = 9;
pub const REG_RBP: usize = 10;
pub const REG_RBX: usize = 11;
pub const REG_RDX: usize = 12;
pub const REG_RAX: usize = 13;
pub const REG_RCX: usize = 14;
pub const REG_RSP: usize = 15;
pub const REG_RIP: usize = 16;
