use crate::hardening::memory;
use crate::secrets::secret::Secret;

use std::sync::Once;
use zeroize::Zeroize;

static SHUTDOWN_ONCE: Once = Once::new();

pub fn secure_shutdown() {
    SHUTDOWN_ONCE.call_once(|| {
        let mut secret = Secret::new();
        secret.as_mut().zeroize();
        memory::unlock_memory();
        std::process::exit(0);
    });
}
