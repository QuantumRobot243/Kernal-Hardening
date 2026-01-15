use crate::hardening::memory;
use crate::secrets::secret;
use std::sync::Once;
use std::process;

static SHUTDOWN_ONCE: Once = Once::new();

pub fn secure_shutdown() {
    SHUTDOWN_ONCE.call_once(|| {
        eprintln!("\n[!] Secure shutdown initiated. Wiping memory...");

        secret::wipe_all_registered_secrets();
        memory::unlock_memory();
        process::exit(0);
    });
}
