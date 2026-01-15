mod hardening;
mod secrets;
mod shutdown;

use hardening::{memory, dump, signals};

fn main() {
    memory::lock_memory();
    dump::disable_core_dumps();
    signals::install_signal_handlers();

    println!("Hardened shell running. Press Ctrl+C to exit.");

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
