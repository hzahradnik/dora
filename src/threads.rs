use parking_lot::{Condvar, Mutex};
use std::cell::RefCell;
use std::ptr;
use std::sync::Arc;

use exception::DoraToNativeInfo;
use gc::{Address, Region};

thread_local! {
    pub static THREAD: RefCell<Arc<DoraThread>> = RefCell::new(DoraThread::new());
}

pub struct Threads {
    threads: Mutex<Vec<Arc<DoraThread>>>,
    cond_join: Condvar,
}

impl Threads {
    pub fn new() -> Threads {
        Threads {
            threads: Mutex::new(Vec::new()),
            cond_join: Condvar::new(),
        }
    }

    pub fn attach_current_thread(&self) {
        THREAD.with(|thread| {
            let mut threads = self.threads.lock();
            threads.push(thread.borrow().clone());
        });
    }

    pub fn attach_thread(&self, thread: Arc<DoraThread>) {
        let mut threads = self.threads.lock();
        threads.push(thread);
    }

    pub fn detach_current_thread(&self) {
        THREAD.with(|thread| {
            let mut threads = self.threads.lock();
            threads.retain(|elem| !Arc::ptr_eq(elem, &*thread.borrow()));
            self.cond_join.notify_all();
        });
    }

    pub fn join_all(&self) {
        let mut threads = self.threads.lock();

        while threads.len() > 0 {
            self.cond_join.wait(&mut threads);
        }
    }
}

pub struct DoraThread {}

impl DoraThread {
    pub fn new() -> Arc<DoraThread> {
        Arc::new(DoraThread {})
    }
}

pub struct ThreadLocalData {
    d2n: *const DoraToNativeInfo,
    tlab_top: Address,
    tlab_end: Address,
    concurrent_marking: bool,
}

impl ThreadLocalData {
    pub fn new() -> ThreadLocalData {
        ThreadLocalData {
            d2n: ptr::null(),
            tlab_top: Address::null(),
            tlab_end: Address::null(),
            concurrent_marking: false,
        }
    }

    pub fn tlab_initialize(&mut self, start: Address, end: Address) {
        self.tlab_top = start;
        self.tlab_end = end;
    }

    pub fn tlab_rest(&self) -> usize {
        self.tlab_end.offset_from(self.tlab_top)
    }

    pub fn tlab_region(&self) -> Region {
        Region::new(self.tlab_top, self.tlab_end)
    }

    pub fn tlab_top_offset() -> i32 {
        offset_of!(ThreadLocalData, tlab_top) as i32
    }

    pub fn tlab_end_offset() -> i32 {
        offset_of!(ThreadLocalData, tlab_end) as i32
    }

    pub fn concurrent_marking_offset() -> i32 {
        offset_of!(ThreadLocalData, concurrent_marking) as i32
    }
}
