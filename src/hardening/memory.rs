use libc::{mlockall, munlockall, MCL_CURRENT, MCL_FUTURE}; 
pub fn lock_memory() { 
    unsafe {
        if mlockall(MCL_CURRENT | MCL_FUTURE) != 0 {
            panic!("mlockall failed");
        }
    }
}

pub fn unlock_memory() {
    unsafe {
        munlockall(); 
    }
}
