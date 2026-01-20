use libc::{
    mount, umount2, unshare, CLONE_NEWNS, CLONE_NEWUSER, MNT_DETACH,
    MS_NOSUID, MS_NOEXEC, MS_NODEV, MS_PRIVATE, MS_REC,
};
use std::ffi::CString;
use std::fs;
use std::process;
use std::ptr;

pub fn isolate_environment() {
    let uid = unsafe { libc::getuid() };
    let gid = unsafe { libc::getgid() };

    unsafe {
        if unshare(CLONE_NEWUSER | CLONE_NEWNS) != 0 {
            process::exit(1);
        }

        fs::write("/proc/self/setgroups", "deny").ok();
        fs::write("/proc/self/uid_map", format!("0 {} 1", uid)).ok();
        fs::write("/proc/self/gid_map", format!("0 {} 1", gid)).ok();

        let root = CString::new("/").unwrap();
        if mount(
            ptr::null(),
            root.as_ptr(),
            ptr::null(),
            MS_PRIVATE | MS_REC,
            ptr::null(),
        ) != 0
        {
            process::exit(1);
        }

        let tmp_path = CString::new("/tmp").unwrap();
        let tmpfs = CString::new("tmpfs").unwrap();
        let flags = MS_NOSUID | MS_NOEXEC | MS_NODEV;

        mount(
            tmpfs.as_ptr(),
            tmp_path.as_ptr(),
            tmpfs.as_ptr(),
            flags,
            ptr::null(),
        );

        let proc_path = CString::new("/proc").unwrap();
        let proc_fs = CString::new("proc").unwrap();

        umount2(proc_path.as_ptr(), MNT_DETACH);
        mount(
            proc_fs.as_ptr(),
            proc_path.as_ptr(),
            proc_fs.as_ptr(),
            flags,
            ptr::null(),
        );
    }
}