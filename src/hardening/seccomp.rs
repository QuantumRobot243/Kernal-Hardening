use syscalls::{Sysno, syscall};

pub fn apply_filters() {
    // example.
    // Ideally, you strictly whitelist only:
    // - read
    // - write
    // - exit_group
    // - sigreturn

