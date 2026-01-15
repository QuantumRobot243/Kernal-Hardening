mod hardening;
mod secrets;
mod shutdown;

use hardening::{memory, dump, signals, anti_debug};
use secrets::secret::Secret; 
use std::panic;

fn main() {
    panic::set_hook(Box::new(|info| {
        eprintln!("\n[CRITICAL] Panic detected: {}", info);
        shutdown::secure_shutdown();
    }));
    anti_debug::block_debugger();
    memory::lock_memory();
    dump::disable_core_dumps();
    signals::install_signal_handlers();

    let mut my_password = Secret::new();

    println!("H-shell active....");

    loop {
        let _ = my_password.data[0];
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
