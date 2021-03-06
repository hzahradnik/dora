use std::collections::HashSet;
use std::fmt;
use std::ptr;
use std::sync::Arc;

use crate::cpu::flush_icache;
use crate::dseg::DSeg;
use crate::gc::Address;
use crate::os;
use crate::ty::SourceTypeArray;
use crate::utils::GrowableVec;
use crate::vm::FctId;
use crate::vm::VM;

use dora_parser::Position;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct JitFctId(usize);

impl JitFctId {
    pub fn from(idx: usize) -> JitFctId {
        JitFctId(idx)
    }

    pub fn idx(self) -> usize {
        self.0
    }
}

impl GrowableVec<JitFct> {
    pub fn idx(&self, index: JitFctId) -> Arc<JitFct> {
        self.idx_usize(index.0)
    }
}

impl From<usize> for JitFctId {
    fn from(data: usize) -> JitFctId {
        JitFctId(data)
    }
}

pub enum JitFct {
    Compiled(Code),
    Uncompiled,
}

impl JitFct {
    pub fn fct_id(&self) -> FctId {
        match self {
            &JitFct::Compiled(ref base) => base.fct_id(),
            &JitFct::Uncompiled => unreachable!(),
        }
    }

    pub fn instruction_start(&self) -> Address {
        match self {
            &JitFct::Compiled(ref base) => base.instruction_start(),
            &JitFct::Uncompiled => unreachable!(),
        }
    }

    pub fn instruction_end(&self) -> Address {
        match self {
            &JitFct::Compiled(ref base) => base.instruction_end(),
            &JitFct::Uncompiled => unreachable!(),
        }
    }

    pub fn ptr_start(&self) -> Address {
        match self {
            &JitFct::Compiled(ref base) => base.ptr_start(),
            &JitFct::Uncompiled => unreachable!(),
        }
    }

    pub fn ptr_end(&self) -> Address {
        match self {
            &JitFct::Compiled(ref base) => base.ptr_end(),
            &JitFct::Uncompiled => unreachable!(),
        }
    }

    pub fn to_code(&self) -> Option<&Code> {
        match self {
            &JitFct::Compiled(ref code) => Some(code),
            &JitFct::Uncompiled => unreachable!(),
        }
    }

    pub fn framesize(&self) -> i32 {
        match self {
            &JitFct::Compiled(ref base) => base.framesize(),
            &JitFct::Uncompiled => unreachable!(),
        }
    }

    pub fn gcpoint_for_offset(&self, offset: u32) -> Option<&GcPoint> {
        match self {
            &JitFct::Compiled(ref base) => base.gcpoint_for_offset(offset),
            &JitFct::Uncompiled => unreachable!(),
        }
    }

    pub fn position_for_offset(&self, offset: u32) -> Option<Position> {
        match self {
            &JitFct::Compiled(ref base) => base.position_for_offset(offset),
            &JitFct::Uncompiled => unreachable!(),
        }
    }

    pub fn comment_for_offset(&self, offset: u32) -> Option<&String> {
        match self {
            &JitFct::Compiled(ref base) => base.comment_for_offset(offset),
            &JitFct::Uncompiled => unreachable!(),
        }
    }

    pub fn lazy_for_offset(&self, offset: u32) -> Option<&LazyCompilationSite> {
        match self {
            &JitFct::Compiled(ref base) => base.lazy_for_offset(offset),
            &JitFct::Uncompiled => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub enum JitDescriptor {
    DoraFct(FctId),
    CompileStub,
    TrapStub,
    AllocStub,
    VerifyStub,
    NativeStub(FctId),
    DoraStub,
    GuardCheckStub,
    SafepointStub,
}

pub struct Code {
    code_start: Address,
    code_end: Address,

    // pointer to beginning of function
    function_entry: Address,

    desc: JitDescriptor,

    framesize: i32,
    lazy_compilation: LazyCompilationData,
    gcpoints: GcPoints,
    comments: Comments,
    positions: PositionTable,
}

impl Code {
    pub fn from_optimized_buffer(vm: &VM, buffer: &[u8], desc: JitDescriptor) -> Code {
        let dseg = DSeg::new();

        Code::from_buffer(
            vm,
            &dseg,
            buffer,
            LazyCompilationData::new(),
            GcPoints::new(),
            0,
            Comments::new(),
            PositionTable::new(),
            desc,
        )
    }

    pub fn from_buffer(
        vm: &VM,
        dseg: &DSeg,
        buffer: &[u8],
        lazy_compilation: LazyCompilationData,
        gcpoints: GcPoints,
        framesize: i32,
        comments: Comments,
        positions: PositionTable,
        desc: JitDescriptor,
    ) -> Code {
        let code_space_size = dseg.size() as usize + buffer.len();
        let code_start = vm.gc.alloc_code(code_space_size);
        let code_end = code_start.offset(code_space_size);

        if code_start.is_null() {
            panic!("out of memory: not enough executable memory left!");
        }

        os::jit_writable();

        dseg.finish(code_start.to_ptr());

        let function_entry = code_start.offset(dseg.size() as usize);

        unsafe {
            ptr::copy_nonoverlapping(buffer.as_ptr(), function_entry.to_mut_ptr(), buffer.len());
        }

        os::jit_executable();

        flush_icache(code_start.to_ptr(), code_space_size);

        Code {
            code_start,
            code_end,
            function_entry,
            desc,
            lazy_compilation,
            gcpoints,
            comments,
            framesize,
            positions,
        }
    }

    pub fn position_for_offset(&self, offset: u32) -> Option<Position> {
        self.positions.get(offset)
    }

    pub fn gcpoint_for_offset(&self, offset: u32) -> Option<&GcPoint> {
        self.gcpoints.get(offset)
    }

    pub fn ptr_start(&self) -> Address {
        self.code_start
    }

    pub fn ptr_end(&self) -> Address {
        self.code_end
    }

    pub fn fct_id(&self) -> FctId {
        match self.desc {
            JitDescriptor::NativeStub(fct_id) => fct_id,
            JitDescriptor::DoraFct(fct_id) => fct_id,
            _ => panic!("no fctid found"),
        }
    }

    pub fn instruction_start(&self) -> Address {
        self.function_entry
    }

    pub fn instruction_end(&self) -> Address {
        self.code_end
    }

    pub fn framesize(&self) -> i32 {
        self.framesize
    }

    pub fn comment_for_offset(&self, offset: u32) -> Option<&String> {
        self.comments.get(offset)
    }

    pub fn lazy_for_offset(&self, offset: u32) -> Option<&LazyCompilationSite> {
        self.lazy_compilation.get(offset)
    }
}

impl fmt::Debug for Code {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "JitBaselineFct {{ start: {:?}, end: {:?}, desc: {:?} }}",
            self.ptr_start(),
            self.ptr_end(),
            self.desc,
        )
    }
}

#[derive(Debug)]
pub struct GcPoints {
    entries: Vec<(u32, GcPoint)>,
}

impl GcPoints {
    pub fn new() -> GcPoints {
        GcPoints {
            entries: Vec::new(),
        }
    }

    pub fn get(&self, offset: u32) -> Option<&GcPoint> {
        let result = self
            .entries
            .binary_search_by_key(&offset, |&(offset, _)| offset);

        match result {
            Ok(idx) => Some(&self.entries[idx].1),
            Err(_) => None,
        }
    }

    pub fn insert(&mut self, offset: u32, gcpoint: GcPoint) {
        if let Some(last) = self.entries.last_mut() {
            debug_assert!(offset > last.0);
        }

        self.entries.push((offset, gcpoint));
    }
}

#[derive(Debug)]
pub struct GcPoint {
    pub offsets: Vec<i32>,
}

impl GcPoint {
    pub fn new() -> GcPoint {
        GcPoint {
            offsets: Vec::new(),
        }
    }

    pub fn merge(lhs: GcPoint, rhs: GcPoint) -> GcPoint {
        let mut offsets = HashSet::new();

        for offset in lhs.offsets {
            offsets.insert(offset);
        }

        for offset in rhs.offsets {
            offsets.insert(offset);
        }

        GcPoint::from_offsets(offsets.drain().collect())
    }

    pub fn from_offsets(offsets: Vec<i32>) -> GcPoint {
        GcPoint { offsets }
    }
}

pub struct Comments {
    entries: Vec<(u32, String)>,
}

impl Comments {
    pub fn new() -> Comments {
        Comments {
            entries: Vec::new(),
        }
    }

    pub fn get(&self, offset: u32) -> Option<&String> {
        let result = self
            .entries
            .binary_search_by_key(&offset, |&(offset, _)| offset);

        match result {
            Ok(idx) => Some(&self.entries[idx].1),
            Err(_) => None,
        }
    }

    pub fn insert(&mut self, offset: u32, comment: String) {
        if let Some(last) = self.entries.last_mut() {
            debug_assert!(offset >= last.0);
        }

        self.entries.push((offset, comment));
    }
}

#[derive(Debug)]
pub struct PositionTable {
    entries: Vec<(u32, Position)>,
}

impl PositionTable {
    pub fn new() -> PositionTable {
        PositionTable {
            entries: Vec::new(),
        }
    }

    pub fn insert(&mut self, offset: u32, position: Position) {
        if let Some(last) = self.entries.last() {
            debug_assert!(offset > last.0);
        }

        self.entries.push((offset, position));
    }

    pub fn get(&self, offset: u32) -> Option<Position> {
        let result = self
            .entries
            .binary_search_by_key(&offset, |&(offset, _)| offset);

        match result {
            Ok(idx) => Some(self.entries[idx].1),
            Err(_) => None,
        }
    }
}

#[derive(Debug)]
pub struct LazyCompilationData {
    entries: Vec<(u32, LazyCompilationSite)>,
}

impl LazyCompilationData {
    pub fn new() -> LazyCompilationData {
        LazyCompilationData {
            entries: Vec::new(),
        }
    }

    pub fn insert(&mut self, offset: u32, info: LazyCompilationSite) {
        if let Some(last) = self.entries.last() {
            debug_assert!(offset > last.0);
        }

        self.entries.push((offset, info));
    }

    pub fn get(&self, offset: u32) -> Option<&LazyCompilationSite> {
        let result = self
            .entries
            .binary_search_by_key(&offset, |&(offset, _)| offset);

        match result {
            Ok(idx) => Some(&self.entries[idx].1),
            Err(_) => None,
        }
    }
}

#[derive(Clone, Debug)]
pub enum LazyCompilationSite {
    Direct(FctId, i32, SourceTypeArray),
    Virtual(bool, FctId, u32, SourceTypeArray),
}
