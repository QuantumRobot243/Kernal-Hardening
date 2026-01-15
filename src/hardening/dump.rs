use libc::{prctl, PR_SET_DUMPABLE};

pub fn disable_core_dumps() {
    unsafe {
        if prctl(PR_SET_DUMPABLE, 0, 0, 0, 0) != 0 {
            panic!("Failed to disable core dumps");
        }
    }
}
